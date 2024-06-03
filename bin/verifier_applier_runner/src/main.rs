use crate::errors::RunnerError;
use crate::models::get_agreements_data;
use clap::Parser;
use run::run;
use starknet::core::types::FieldElement;
use url::Url;

mod declare;
mod deploy;
mod errors;
mod get_account;
mod invoke;
mod models;
mod receipt;
mod run;

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long, short, env, default_value = "0x534e5f5345504f4c4941")]
    chain_id: FieldElement,

    #[arg(long, short, env)]
    address: FieldElement,

    #[arg(long, short, env)]
    private_key: FieldElement,

    #[arg(long, short, env, default_value = "0")]
    salt: FieldElement,

    #[arg(
        long,
        short,
        env,
        default_value = "0x41A78E741E5AF2FEC34B695679BC6891742439F7AFB8484ECD7766661AD02BF"
    )]
    udc_address: FieldElement,

    #[arg(long, env, default_value = "http://localhost:5050/rpc")]
    rpc_url: Url,

    #[arg(long, env, default_value = "http://prover.visoft.dev:3618")]
    prover_url: Url,

    #[arg(long, env)]
    prover_secret_key: String,

    #[arg(long, env)]
    program_hash: FieldElement,
}

#[tokio::main]
async fn main() -> Result<(), RunnerError> {
    let args: Args = Args::parse();
    let (agreements, client_public_key, server_public_key) =
        get_agreements_data("target/generator_output/in.json")?;
    run(args, agreements, server_public_key, client_public_key).await?;

    Ok(())
}
