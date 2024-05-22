// use std::{thread::sleep, time::Duration};

// use starknet::macros::felt;

// use crate::{
//     apply::apply_agreements, deploy::deploy_contract, errors::RunnerError,
//     get_account::get_account, models::FieldElementAgreement, Args,
// };

// pub(crate) async fn sepolia_run(
//     args: Args,
//     agreements: Vec<FieldElementAgreement>,
//     server_public_key: String,
//     client_public_key: String,
// ) -> Result<(), RunnerError> {
//     let prefunded_account = get_account(
//         args.rpc_url.clone(),
//         args.chain_id,
//         args.address,
//         args.private_key,
//     );
//     println!("GOT ACCOUNT");
//     let class_hash = felt!("0x026c4d6961674f8c33c55d2f7c9e78c32d00e73552bd0c1df8652db0b42bdd9c");
//     let deployment_address = deploy_contract(
//         prefunded_account,
//         client_public_key,
//         server_public_key,
//         class_hash,
//         args.salt,
//         args.udc_address,
//     )
//     .await?;
//     println!("DEPLOYED NEW CONTRACT");
//     const DELAY_SECONDS: u64 = 10;

//     println!("Waiting for contract deployment...");
//     sleep(Duration::new(DELAY_SECONDS, 0));
//     println!("contract deployment...");
//     let gas_sum = apply_agreements(
//         agreements.clone(),
//         deployment_address.deployed_address,
//         args.rpc_url,
//         args.chain_id,
//         args.address,
//         args.private_key,
//     )
//     .await?;
//     println!(
//         "Gas consumed by {} contracts: : {}",
//         agreements.len(),
//         gas_sum
//     );
//     Ok(())
// }
