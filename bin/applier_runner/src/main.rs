use apply::apply_agreements;
use clap::Parser;
use starknet::core::types::FieldElement;
use tokio::time::Instant;
use tracing_subscriber::FmtSubscriber;
use utils::{
    account::get_account, args::Args, declare::declare_contract, deploy::deploy_contract,
    models::get_agreements_data, runner_error::RunnerError,
};
mod apply;

#[tokio::main]
async fn main() -> Result<(), RunnerError> {
    let args: Args = Args::parse();
    let (agreements, client_public_key, server_public_key) =
        get_agreements_data("target/generator_output/in.json")?;
    let subscriber = FmtSubscriber::builder().finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let prefunded_account = get_account(
        args.rpc_url.clone(),
        args.chain_id,
        args.address,
        args.private_key,
    );

    let class_hash: FieldElement = declare_contract(
        &prefunded_account,
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
