use std::time::Instant;

use crate::{
    apply::apply_agreements, declare::declare_contract, deploy::deploy_contract_on_devnet,
    errors::RunnerError, get_account::get_account, models::FieldElementAgreement, Args,
};
use starknet::core::types::FieldElement;

pub(crate) async fn devnet_run(
    args: Args,
    agreements: Vec<FieldElementAgreement>,
    server_public_key: String,
    client_public_key: String,
) -> Result<(), RunnerError> {
    println!("DECLARED CONTRACT");
    let get_account = get_account(
        args.rpc_url_devnet.clone(),
        args.chain_id,
        args.address_devnet,
        args.private_key_devnet,
    );
    let prefunded_account = get_account;
    let class_hash: FieldElement = declare_contract(&prefunded_account).await?;
    println!("DECLARED CONTRACT");
    let deployed_address = deploy_contract_on_devnet(
        args.clone(),
        client_public_key,
        server_public_key,
        class_hash,
        args.salt_devnet,
        args.udc_address,
    )
    .await?;
    println!("DEPLOYED CONTRACT {:x}", deployed_address);

    println!("contract deployment...");
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

    println!(
        "Gas consumed by {} contracts: : {}",
        agreements.len(),
        gas_sum
    );
    println!("Time taken to execute apply_agreements: {:?}", duration);

    Ok(())
}
