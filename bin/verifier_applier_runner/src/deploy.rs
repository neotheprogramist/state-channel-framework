use crate::get_account::get_account;
use crate::{errors::RunnerError, Args};
use anyhow::anyhow;
use sncast::{
    handle_wait_for_tx, response::errors::StarknetCommandError, ValidatedWaitParams, WaitForTx,
};
use starknet::accounts::AccountError::Provider;
use starknet::{
    accounts::{AccountError, ConnectedAccount, SingleOwnerAccount},
    contract::ContractFactory,
    core::types::{FieldElement, StarknetError},
    providers::{jsonrpc::HttpTransport, JsonRpcClient, ProviderError},
    signers::LocalWallet,
};

pub async fn deploy_contract(
    args: Args,
    class_hash: FieldElement,
    salt: FieldElement,
    udc_address: FieldElement,
    calldata: Vec<FieldElement>,
) -> Result<FieldElement, RunnerError> {
    let prefunded_account = get_account(
        args.rpc_url.clone(),
        args.chain_id,
        args.address,
        args.private_key,
    );

    let contract_factory = create_contract_factory(class_hash, prefunded_account, udc_address);

    let deployment = contract_factory.deploy(calldata, salt, true);
    let prefunded_account = get_account(
        args.rpc_url.clone(),
        args.chain_id,
        args.address,
        args.private_key,
    );
    let result = match deployment.send().await {
        Ok(result) => handle_wait_for_tx(
            prefunded_account.provider(),
            result.transaction_hash,
            deployment.deployed_address(),
            get_wait_config(false, 5),
        )
        .await
        .map_err(StarknetCommandError::from),
        Err(AccountError::Provider(ProviderError::StarknetError(
            StarknetError::ContractError(data),
        ))) => {
            println!("StarknetError encountered: {}", data.revert_error); // Debugging print
            return Err(RunnerError::AccountFailure(format!(
                "Contract error: {}",
                data.revert_error
            )));
        }
        Err(Provider(error)) => Err(StarknetCommandError::ProviderError(error.into())),
        _ => Err(anyhow!("Unknown RPC error").into()),
    };

    match result {
        Ok(deployed_address) => Ok(deployed_address),
        Err(_) => Err(RunnerError::DeploymentFailure(
            "Failed to deploy contract".to_string(),
        )),
    }
}

pub fn get_wait_config(wait: bool, retry_interval: u8) -> WaitForTx {
    let waiter_params = ValidatedWaitParams::new(retry_interval, 60);
    WaitForTx {
        wait,
        wait_params: waiter_params,
    }
}

fn create_contract_factory(
    class_hash: FieldElement,
    prefunded_account: SingleOwnerAccount<JsonRpcClient<HttpTransport>, LocalWallet>,
    udc_address: FieldElement,
) -> ContractFactory<SingleOwnerAccount<JsonRpcClient<HttpTransport>, LocalWallet>> {
    ContractFactory::new_with_udc(class_hash, prefunded_account, udc_address)
}
