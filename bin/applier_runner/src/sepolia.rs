use std::time::Instant;

use starknet::core::types::FieldElement;

use crate::{
    apply::apply_agreements, declare::declare_contract, deploy::deploy_contract_on_sepolia,
    errors::RunnerError, get_account::get_account, models::Agreement, Args,
};

pub(crate) async fn sepolia_run(
    args: Args,
    agreements: Vec<Agreement>,
    server_public_key: FieldElement,
    client_public_key: FieldElement,
) -> Result<(), RunnerError> {
    let class_hash: starknet::core::types::FieldElement = args.declared_contract_address;
    let prefunded_account = get_account(
        args.rpc_url.clone(),
        args.chain_id,
        args.address,
        args.private_key,
    );
    tracing::info!("prefunded_account CONTRACCT");

    let class_hash: FieldElement = declare_contract(&prefunded_account).await?;
    tracing::info!("DECLARED CONTRACCT");

    let deployed_address = deploy_contract_on_sepolia(
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

    tracing::info!(
        "Gas consumed by {} contracts: : {}",
        agreements.len(),
        gas_sum
    );
    tracing::info!("Time taken to execute apply_agreements: {:?}", duration);

    Ok(())
}
