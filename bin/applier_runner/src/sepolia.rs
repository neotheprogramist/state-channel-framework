use std::time::Instant;

use starknet::macros::felt;

use crate::{
    apply::apply_agreements, deploy::deploy_contract_on_sepolia, errors::RunnerError,
    models::FieldElementAgreement, Args,
};

pub(crate) async fn sepolia_run(
    args: Args,
    agreements: Vec<FieldElementAgreement>,
    server_public_key: String,
    client_public_key: String,
) -> Result<(), RunnerError> {
    let class_hash: starknet::core::types::FieldElement = args.declared_contract_address;

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

    println!(
        "Gas consumed by {} contracts: : {}",
        agreements.len(),
        gas_sum
    );
    println!("Time taken to execute apply_agreements: {:?}", duration);

    Ok(())
}
