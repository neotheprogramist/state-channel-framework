use std::time::Instant;

use crate::{
    apply::apply_agreements, declare::declare_contract, deploy::deploy_contract,
    errors::RunnerError, get_account::get_account, models::FieldElementAgreement, Args,
};
use starknet::core::types::FieldElement;

pub(crate) async fn run(
    args: Args,
    agreements: Vec<FieldElementAgreement>,
    server_public_key: String,
    client_public_key: String,
) -> Result<(), RunnerError> {
    let get_account = get_account(
        args.rpc_url.clone(),
        args.chain_id,
        args.address,
        args.private_key,
    );
    let prefunded_account: starknet::accounts::SingleOwnerAccount<
        starknet::providers::JsonRpcClient<starknet::providers::jsonrpc::HttpTransport>,
        starknet::signers::LocalWallet,
    > = get_account;
    let class_hash: FieldElement = declare_contract(
        &prefunded_account,
        "../../../target/dev/applier_Applier.contract_class.json",
        "../../../target/dev/applier_Applier.compiled_contract_class.json",
    ).await?;

    let deployed_address = deploy_contract(
        args.clone(),
        client_public_key,
        server_public_key,
        class_hash,
        args.salt,
        args.udc_address,
    )
    .await?;

    let start = Instant::now();

    let gas_sum = apply_agreements(
        agreements.clone(),
        deployed_address,
        args.rpc_url,
        args.chain_id,
        args.address,
        args.private_key,
    )
    .await?;
    let duration = start.elapsed();

    println!(
        "Gas consumed by {} contracts: : {}",
        agreements.len(),
        gas_sum
    );
    println!("Time taken to execute apply_agreements: {:?}", duration);

    Ok(())
}
