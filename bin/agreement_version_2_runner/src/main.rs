use crate::apply::apply_agreements;
use crate::declare::declare_contract;
use crate::deploy::deploy_contract;
use crate::errors::RunnerError;
use crate::models::get_agreements_data;
use clap::Parser;
use starknet::core::types::FieldElement;
use tracing::field::Field;
use url::Url;
mod apply;
mod declare;
mod deploy;
mod errors;
mod get_account;
mod models;
use get_account::get_account;
use std::thread::sleep;
use std::time::{Duration, Instant};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long, short, env)]
    rpc_url: Url,

    #[arg(long, short, env)]
    chain_id: FieldElement,

    #[arg(long, short, env)]
    address: FieldElement,

    #[arg(long, short, env)]
    private_key: FieldElement,

    #[arg(long, short, env)]
    udc_address: FieldElement,

    #[arg(long, short, env)]
    salt: FieldElement,
}
use starknet::macros::felt;
#[tokio::main]
async fn main() -> Result<(), RunnerError> {
    let args: Args = Args::parse();
    let (agreements, client_public_key, server_public_key) = get_agreements_data()?;

    let prefunded_account = get_account(
        args.rpc_url.clone(),
        args.chain_id,
        args.address,
        args.private_key,
    );
    println!("GOT ACCOUNT");
    // let class_hash: FieldElement = declare_contract(&prefunded_account).await?;
    println!("DECLARED CONTRACT");
    let class_hash = felt!("0x026c4d6961674f8c33c55d2f7c9e78c32d00e73552bd0c1df8652db0b42bdd9c");
    let deployment_address = deploy_contract(
        prefunded_account,
        client_public_key,
        server_public_key,
        class_hash,
        args.salt,
        args.udc_address,
    )
    .await?;
    println!("DEPLOYED CONTRACT");
    // let deployment_address =
    //     felt!("0xfbcda98d3bad311cee270f638652408b85740ed2861c939f89331a4bf8ca50");

    const DELAY_SECONDS: u64 = 10;
    // let mut deployed = false;
    // for _ in 0..RETRY_COUNT {
    //     if check_contract_deployed(deployment_address.deployed_address).await {
    //         deployed = true;
    //         break;
    //     }
    //     println!("Waiting for contract deployment...");
    //     sleep(Duration::new(DELAY_SECONDS, 0));
    // }
    // if !deployed {
    //     println!("Failed to confirm contract deployment within the expected time.");
    //     return Err(RunnerError::AccountFailure(
    //         "Failed to confirm contract deployment within the expected time".to_string(),
    //     ));
    // }
    println!("Waiting for contract deployment...");
    sleep(Duration::new(DELAY_SECONDS, 0));
    println!("cntract deployment...");
    let gas_sum = apply_agreements(
        agreements.clone(),
        deployment_address.deployed_address,
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

    println!("FINISHED");

    Ok(())
}

async fn check_contract_deployed(deployed_address: FieldElement) -> bool {
    // Implement a function that checks if the contract is deployed on the network
    // This function should return true if the contract is deployed, false otherwise
    // Example:
    // let deployment_status = check_deployment_status(deployed_address).await;
    // deployment_status.is_deployed()

    // Placeholder for actual deployment check logic
    true
}
