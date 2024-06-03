use crate::errors::RunnerError;
use crate::models::get_agreements_data;
use apply::apply_agreements;
use clap::Parser;
use declare::declare_contract;
use deploy::deploy_contract;
use starknet::core::types::FieldElement;
use tokio::time::Instant;
use tracing_subscriber::FmtSubscriber;
use url::Url;
mod account;
mod apply;
mod declare;
mod deploy;
mod errors;
mod models;

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long, short, env, default_value = "0x534e5f5345504f4c4941")]
    chain_id: FieldElement,

    #[arg(
        long,
        short,
        env,
        default_value = "0x41A78E741E5AF2FEC34B695679BC6891742439F7AFB8484ECD7766661AD02BF"
    )]
    udc_address: FieldElement,

    #[arg(
        long,
        short,
        env,
        default_value = "0x023ba0a5877c0e8772fc22a60c02a5fe5fddd592a8a47079522667c04418c29d"
    )]
    salt: FieldElement,

    #[arg(long, short, env)]
    address: FieldElement,

    #[arg(long, short, env)]
    private_key: FieldElement,

    #[arg(long, env, default_value = "http://localhost:5050/rpc")]
    rpc_url: Url,
}

#[tokio::main]
async fn main() -> Result<(), RunnerError> {
    let args: Args = Args::parse();
    let (agreements, client_public_key, server_public_key) =
        get_agreements_data("target/generator_output/in.json")?;
    let subscriber = FmtSubscriber::builder().finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let prefunded_account = account::get_account(
        args.rpc_url.clone(),
        args.chain_id,
        args.address,
        args.private_key,
    );

    let class_hash: FieldElement = declare_contract(
        prefunded_account,
        "target/dev/applier_Applier.contract_class.json",
        "target/dev/applier_Applier.compiled_contract_class.json",
    )
    .await?;
    tracing::info!("Declared contract");

    let deployed_address = deploy_contract(
        client_public_key,
        server_public_key,
        class_hash,
        args.clone(),
    )
    .await?;
    tracing::info!("Deployed contract");

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

    tracing::info!(
        "Gas consumed by {} contracts: : {}",
        agreements.len(),
        gas_sum
    );
    tracing::info!("Time taken to execute apply_agreements: {:?}", duration);

    Ok(())
}
