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
    println!("GOT ACCOUNT");

    let class_hash = felt!("0x026c4d6961674f8c33c55d2f7c9e78c32d00e73552bd0c1df8652db0b42bdd9c");

    let deployed_address = deploy_contract_on_sepolia(
        args.clone(),
        client_public_key,
        server_public_key,
        class_hash,
        args.salt,
        args.udc_address,
    )
    .await?;
    println!("DEPLOYED NEW CONTRACT");
    let gas_sum = apply_agreements(
        agreements.clone(),
        deployed_address,
        args.rpc_url,
        args.chain_id,
        args.address,
        args.private_key,
    )
    .await?;
    println!(
        "Gas consumed by {} contracts: : {}",
        agreements.len(),
        gas_sum
    );
    Ok(())
}
