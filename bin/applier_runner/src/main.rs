use crate::devnet::devnet_run;
use crate::errors::RunnerError;
use crate::models::get_agreements_data;
use clap::Parser;
use sepolia::sepolia_run;
use starknet::core::types::FieldElement;
use tracing_subscriber::{EnvFilter, FmtSubscriber};
use url::Url;
mod apply;
mod declare;
mod deploy;
pub mod devnet;
mod errors;
mod get_account;
mod models;
mod sepolia;
use dialoguer::{theme::ColorfulTheme, Select};

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long, short, env, default_value = "0x534e5f5345504f4c4941")]
    chain_id: FieldElement,

    #[arg(long, short, env)]
    address: FieldElement,

    #[arg(long, short, env)]
    private_key: FieldElement,
    #[arg(
        long,
        short,
        env,
        default_value = "0x023ba0a5877c0e8772fc22a60c02a5fe5fddd592a8a47079522667c04418c29d"
    )]
    salt: FieldElement,

    #[arg(
        long,
        short,
        env,
        default_value = "0x41A78E741E5AF2FEC34B695679BC6891742439F7AFB8484ECD7766661AD02BF"
    )]
    udc_address: FieldElement,

    #[arg(long, env)]
    address_devnet: FieldElement,

    #[arg(long, env, default_value = "http://localhost:5050/rpc")]
    rpc_url_devnet: Url,

    #[arg(
        long,
        env,
        default_value = "https://free-rpc.nethermind.io/sepolia-juno/v0_7"
    )]
    rpc_url: Url,

    #[arg(long, env)]
    private_key_devnet: FieldElement,

    #[arg(long, default_value = "0xcca64674ab8db572")]
    salt_devnet: FieldElement,

    #[arg(
        long,
        env,
        default_value = "0x026c4d6961674f8c33c55d2f7c9e78c32d00e73552bd0c1df8652db0b42bdd9c"
    )]
    declared_contract_address: FieldElement,
}

#[tokio::main]
async fn main() -> Result<(), RunnerError> {
    let args: Args = Args::parse();
    let (agreements, client_public_key, server_public_key) = get_agreements_data()?;
    let subscriber = FmtSubscriber::builder().finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let selections = vec!["Sepolia", "Devnet"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select the network you want to proceed with")
        .default(0)
        .items(&selections)
        .interact()?;

    match selections[selection] {
        "Sepolia" => {
            tracing::info!("You selected Sepolia. Proceeding with Sepolia...");
            let _ = sepolia_run(args, agreements, server_public_key, client_public_key).await;
        }
        "Devnet" => {
            tracing::info!("You selected Devnet. Proceeding with Devnet...");
            let _ = devnet_run(args, agreements, server_public_key, client_public_key).await;
        }
        _ => unreachable!(),
    }

    Ok(())
}
