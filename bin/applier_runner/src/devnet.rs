use std::time::Instant;

use crate::{
    apply::apply_agreements, declare::declare_contract, deploy::deploy_contract_on_devnet,
    errors::RunnerError, get_account::get_account, models::Agreement, Args,
};
use starknet::core::types::FieldElement;

pub(crate) async fn devnet_run(
    args: Args,
    agreements: Vec<Agreement>,
    server_public_key: FieldElement,
    client_public_key: FieldElement,
) -> Result<(), RunnerError> {
    let get_account = get_account(
        args.rpc_url_devnet.clone(),
        args.chain_id,
        args.address_devnet,
        args.private_key_devnet,
    );
    let prefunded_account: starknet::accounts::SingleOwnerAccount<
        starknet::providers::JsonRpcClient<starknet::providers::jsonrpc::HttpTransport>,
        starknet::signers::LocalWallet,
    > = get_account;
    let class_hash: FieldElement = declare_contract(&prefunded_account).await?;
    tracing::info!("DECLARED CONTRACCT");
    let deployed_address = deploy_contract_on_devnet(
        args.clone(),
        client_public_key,
        server_public_key,
        class_hash,
        args.salt_devnet,
        args.udc_address,
    )
    .await?;
tracing::info!("deploued CONTRACCT");

    let start = Instant::now();

    let gas_sum = apply_agreements(
        agreements.clone(),
        deployed_address,
        args.rpc_url_devnet,
        args.chain_id,
        args.address_devnet,
        args.private_key_devnet,
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
