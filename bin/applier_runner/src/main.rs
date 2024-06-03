use clap::Parser;
use starknet::core::types::FieldElement;
use tokio::time::Instant;
use tracing_subscriber::FmtSubscriber;
use utils::{
    account::get_account,
    args::Args,
    declare::declare_contract,
    deploy::deploy_contract,
    invoke::invoke,
    models::get_agreements_data,
    receipt::{extract_gas_fee, wait_for_receipt},
    runner_error::RunnerError,
};

#[tokio::main]
async fn main() -> Result<(), RunnerError> {
    let args: Args = Args::parse();
    let (agreements, client_public_key, server_public_key) =
        get_agreements_data("target/generator_output/in.json")?;
    let subscriber = FmtSubscriber::builder().finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let prefunded_account = get_account(
        args.rpc_url.clone(),
        args.chain_id,
        args.address,
        args.private_key,
    );

    let class_hash: FieldElement = declare_contract(
        &prefunded_account,
        "target/dev/applier_Applier.contract_class.json",
        "target/dev/applier_Applier.compiled_contract_class.json",
    )
    .await?;
    tracing::info!("Declared contract");

    let calldata = vec![client_public_key, server_public_key];

    let deployed_address = deploy_contract(calldata, class_hash, args.clone()).await?;
    tracing::info!("Deployed contract");
    let mut gas_sum: FieldElement = FieldElement::ZERO;

    let start = Instant::now();

    for agreement in agreements.iter() {
        let invoke = invoke(
            &prefunded_account,
            deployed_address,
            "apply",
            vec![
                agreement.quantity,
                agreement.nonce,
                agreement.price,
                agreement.server_signature_r,
                agreement.server_signature_s,
                agreement.client_signature_r,
                agreement.client_signature_s,
            ],
        );
        let tx_hash = invoke.await.unwrap();
        gas_sum +=
            extract_gas_fee(&wait_for_receipt(&prefunded_account, tx_hash).await.unwrap()).unwrap();
    }

    let duration = start.elapsed();

    tracing::info!(
        "Gas consumed by {} contracts: : {}",
        agreements.len(),
        gas_sum
    );
    tracing::info!("Time taken to execute apply_agreements: {:?}", duration);

    Ok(())
}
