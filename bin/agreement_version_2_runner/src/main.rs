use crate::devnet::devnet_run;
use crate::errors::RunnerError;
use crate::models::get_agreements_data;
use clap::Parser;
use sepolia::sepolia_run;
use starknet::core::types::FieldElement;
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
