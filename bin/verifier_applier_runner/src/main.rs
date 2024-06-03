use clap::Parser;
use run::run;

use utils::{args::Args, models::get_agreements_data, runner_error::RunnerError};

mod run;

#[tokio::main]
async fn main() -> Result<(), RunnerError> {
    let args: Args = Args::parse();
    let (agreements, client_public_key, server_public_key) =
        get_agreements_data("target/generator_output/in.json")?;
    run(args, agreements, server_public_key, client_public_key).await?;

    Ok(())
}
