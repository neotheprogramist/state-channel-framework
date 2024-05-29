use crate::get_account::get_account;
use crate::{
    errors::{parse_contract_address_from_error, RunnerError},
    models::AgreementConstructor,
    Args,
};
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

pub async fn deploy_contract_on_sepolia(
    args: Args,
    client_public_key: String,
    server_public_key: String,
    class_hash: FieldElement,
    salt: FieldElement,
    udc_address: FieldElement,
) -> Result<FieldElement, RunnerError> {
    let prefunded_account = get_account(
        args.rpc_url.clone(),
        args.chain_id,
        args.address,
        args.private_key,
    );
    let contract_factory = create_contract_factory(class_hash, prefunded_account, udc_address);
    let agreement_constructor =
        create_agreement_constructor(&client_public_key, &server_public_key)?;

    let deployment = contract_factory.deploy(
        vec![
            agreement_constructor.client_public_key,
            agreement_constructor.server_public_key,
        ],
        salt,
        false,
    );

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
            get_wait_config(true, 5),
        )
        .await
        .map_err(StarknetCommandError::from),
        Err(AccountError::Provider(ProviderError::StarknetError(
            StarknetError::ContractError(data),
        ))) => {
            println!("StarknetError encountered: {}", data.revert_error);
            if data.revert_error.contains("is unavailable for deployment") {
                Ok(parse_contract_address_from_error(&data.revert_error))
            } else {
                return Err(RunnerError::AccountFailure(format!(
                    "Contract error: {}",
                    data.revert_error
                )));
            }
        }
        Err(Provider(error)) => Err(StarknetCommandError::ProviderError(error.into())),
        _ => Err(anyhow!("Unknown RPC error").into()),
    };

    match result {
        Ok(deployed_address) => Ok(deployed_address),
        Err(e) => {
            println!("Failed to deploy contract: {:?}", e);
            Err(RunnerError::DeploymentFailure(
                "Failed to deploy contract".to_string(),
            ))
        }
    }
}

pub async fn deploy_contract_on_devnet(
    args: Args,
    client_public_key: String,
    server_public_key: String,
    class_hash: FieldElement,
    salt: FieldElement,
    udc_address: FieldElement,
) -> Result<FieldElement, RunnerError> {
    let prefunded_account = get_account(
        args.rpc_url_devnet.clone(),
        args.chain_id,
        args.address_devnet,
        args.private_key_devnet,
    );

    let contract_factory = create_contract_factory(class_hash, prefunded_account, udc_address);
    let agreement_constructor =
        create_agreement_constructor(&client_public_key, &server_public_key)?;

    let deployment = contract_factory.deploy(
        vec![
            agreement_constructor.client_public_key,
            agreement_constructor.server_public_key,
        ],
        salt,
        false,
    );
    let prefunded_account = get_account(
        args.rpc_url_devnet.clone(),
        args.chain_id,
        args.address_devnet,
        args.private_key_devnet,
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
            if data.revert_error.contains("is unavailable for deployment") {
                Ok(parse_contract_address_from_error(&data.revert_error))
            } else {
                return Err(RunnerError::AccountFailure(format!(
                    "Contract error: {}",
                    data.revert_error
                )));
            }
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

fn create_agreement_constructor(
    client_public_key: &str,
    server_public_key: &str,
) -> Result<AgreementConstructor, RunnerError> {
    Ok(AgreementConstructor {
        client_balance: 1000000u64.into(),
        server_balance: 1000000u64.into(),
        client_public_key: FieldElement::from_dec_str(client_public_key)?,
        server_public_key: FieldElement::from_dec_str(server_public_key)?,
        a: 0u64.into(),
        b: 0u64.into(),
    })
}
