use std::time::Instant;

use crate::Args;
use cairo_proof_parser::{output::extract_output, parse};
use prover_sdk::{Cairo0ProverInput, ProverAccessKey, ProverSDK};
use serde::Serialize;
use starknet::core::types::FieldElement;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use utils::{
    account::get_account,
    declare::declare_contract,
    deploy::deploy_contract,
    invoke::invoke,
    models::Agreement,
    receipt::{extract_gas_fee, wait_for_receipt},
    runner_error::RunnerError,
};
pub(crate) async fn run(
    args: Args,
    agreements: Vec<Agreement>,
    server_public_key: FieldElement,
    client_public_key: FieldElement,
) -> Result<(), RunnerError> {
    let prefunded_account = get_account(
        args.rpc_url.clone(),
        args.chain_id,
        args.address,
        args.private_key,
    );

    println!("declaring verifier...");
    let fact_registry_class_hash: FieldElement = declare_contract(
        &prefunded_account,
        "target/dev/verifier_FactRegistry.contract_class.json",
        "target/dev/verifier_FactRegistry.compiled_contract_class.json",
    )
    .await?;

    println!("deploying verifier...");
    let fact_registry_address = deploy_contract(
        client_public_key,
        server_public_key,
        fact_registry_class_hash,
        args.clone(),
    )
    .await?;

    println!("declaring applier...");

    let verifier_applier_class_hash: FieldElement = declare_contract(
        &prefunded_account,
        "target/dev/verifier_applier_VerifierApplier.contract_class.json",
        "target/dev/verifier_applier_VerifierApplier.compiled_contract_class.json",
    )
    .await?;

    println!("deploying applier...");
    let verifier_applier_address = deploy_contract(
        client_public_key,
        server_public_key,
        verifier_applier_class_hash,
        args.clone(),
    )
    .await?;

    let program_input = ProgramInput {
        client_public_key,
        server_public_key,
        agreements: agreements.clone(),
        settlement_price: FieldElement::from(1500_u64),
    };

    let mut file: tokio::fs::File = tokio::fs::File::open("target/cairo0/program.casm.json")
        .await
        .map_err(|e| RunnerError::ReadFileError(e.to_string()))?;
    let mut program = String::default();
    file.read_to_string(&mut program)
        .await
        .map_err(|e| RunnerError::ReadFileError(e.to_string()))?;

    let prover_input = Cairo0ProverInput {
        program: serde_json::from_str(&program).unwrap(),
        program_input: serde_json::to_value(program_input).unwrap(),
        layout: "starknet".to_string(),
    };

    let mut file: tokio::fs::File =
        tokio::fs::File::create("target/cairo0/compiled_with_input.json")
            .await
            .map_err(|e| RunnerError::ReadFileError(e.to_string()))?;
    file.write_all(
        serde_json::to_string_pretty(&prover_input)
            .unwrap()
            .as_bytes(),
    )
    .await
    .unwrap();
    file.flush().await.unwrap();

    let mut gas_sum: FieldElement = FieldElement::ZERO;

    let start = Instant::now();

    let secret_key = ProverAccessKey::from_hex_string(&args.clone().prover_secret_key).unwrap();
    let sdk = ProverSDK::new(secret_key, args.clone().prover_url)
        .await
        .unwrap();
    println!("proving...");
    let proof = sdk.prove_cairo0(prover_input).await.unwrap();
    let extracted_program_output = extract_output(&proof).unwrap();
    let calldata: Vec<FieldElement> = parse(&proof).unwrap().into();
    let tx_hash = invoke(
        &prefunded_account,
        fact_registry_address,
        "verify_and_register_fact",
        calldata,
    )
    .await
    .unwrap();
    gas_sum +=
        extract_gas_fee(&wait_for_receipt(&prefunded_account, tx_hash).await.unwrap()).unwrap();

    let tx_hash = invoke(
        &prefunded_account,
        verifier_applier_address,
        "settle",
        extracted_program_output.program_output,
    )
    .await
    .unwrap();
    gas_sum +=
        extract_gas_fee(&wait_for_receipt(&prefunded_account, tx_hash).await.unwrap()).unwrap();

    let duration = start.elapsed();

    println!(
        "Gas consumed by {} contracts: : {}",
        agreements.len(),
        gas_sum
    );
    println!("Time taken to execute apply_agreements: {:?}", duration);

    Ok(())
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgramInput {
    pub client_public_key: FieldElement,
    pub server_public_key: FieldElement,
    pub agreements: Vec<Agreement>,
    pub settlement_price: FieldElement,
}

#[derive(Debug, Serialize)]
pub struct MergedProgramInput {
    pub program: serde_json::Value,
    pub program_input: ProgramInput,
}
