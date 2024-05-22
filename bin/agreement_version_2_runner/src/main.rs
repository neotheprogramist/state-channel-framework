use crate::apply::apply_agreements;
use crate::deploy::deploy_contract_on_sepolia;
use crate::devnet::devnet_run;
use crate::errors::RunnerError;
use crate::models::get_agreements_data;
use clap::Parser;
use models::FieldElementAgreement;
use starknet::core::types::FieldElement;
use url::Url;
mod apply;
mod declare;
mod deploy;
pub mod devnet;
mod errors;
mod get_account;
mod models;
use dialoguer::{theme::ColorfulTheme, Select};
use get_account::get_account;
use std::thread::sleep;
use std::time::Duration;

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long, short, env)]
    chain_id: FieldElement,

    #[arg(long, short, env)]
    address: FieldElement,

    #[arg(long, short, env)]
    rpc_url: Url,

    #[arg(long, short, env)]
    private_key: FieldElement,

    #[arg(long, short, env)]
    salt: FieldElement,

    #[arg(long, short, env)]
    udc_address: FieldElement,

    #[arg(long, env)]
    address_devnet: FieldElement,

    #[arg(long, env)]
    rpc_url_devnet: Url,

    #[arg(long, env)]
    private_key_devnet: FieldElement,

    #[arg(long, env)]
    salt_devnet: FieldElement,
}
use starknet::macros::felt;

#[tokio::main]
async fn main() -> Result<(), RunnerError> {
    let args: Args = Args::parse();
    let (agreements, client_public_key, server_public_key) = get_agreements_data()?;

    let selections = vec!["Sepolia", "Devnet"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select the network you want to proceed with")
        .default(0)
        .items(&selections)
        .interact()?;

    match selections[selection] {
        "Sepolia" => {
            let _ = sepolia_run(args, agreements, server_public_key, client_public_key).await;
        }
        "Devnet" => {
            println!("You selected Devnet. Proceeding with Devnet...");
            let _ = devnet_run(args, agreements, server_public_key, client_public_key).await;
        }
        _ => unreachable!(),
    }

    Ok(())
}

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
    const DELAY_SECONDS: u64 = 10;

    println!("Waiting for contract deployment...");
    sleep(Duration::new(DELAY_SECONDS, 0));
    println!("cntract deployment...");
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
