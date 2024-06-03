use clap::Parser;
use starknet::core::types::FieldElement;
use url::Url;

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(long, short, env, default_value = "0x534e5f5345504f4c4941")]
    pub chain_id: FieldElement,

    #[arg(long, short, env)]
    pub address: FieldElement,

    #[arg(long, short, env)]
    pub private_key: FieldElement,

    #[arg(long, short, env, default_value = "0")]
    pub salt: FieldElement,

    #[arg(
        long,
        short,
        env,
        default_value = "0x41A78E741E5AF2FEC34B695679BC6891742439F7AFB8484ECD7766661AD02BF"
    )]
    pub udc_address: FieldElement,

    #[arg(long, env, default_value = "http://localhost:5050/rpc")]
    pub rpc_url: Url,

    #[arg(long, env, default_value = "http://prover.visoft.dev:3618")]
    pub  prover_url: Url,

    #[arg(long, env)]
    pub prover_secret_key: String,

    #[arg(long, env)]
    pub program_hash: FieldElement,
}
