use crate::account::get_account;
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
use starknet::accounts::ConnectedAccount;
use starknet::{
    accounts::AccountError,
    contract::ContractFactory,
    core::types::{FieldElement, StarknetError},
    providers::ProviderError,
};

pub async fn deploy_contract(
    args: Args,
    client_public_key: FieldElement,
    server_public_key: FieldElement,
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
    let contract_factory =
        ContractFactory::new_with_udc(class_hash, prefunded_account, udc_address);

    let agreement_constructor = AgreementConstructor {
        client_balance: 1000000u64.into(),
        server_balance: 1000000u64.into(),
        client_public_key,
        server_public_key,
        a: 0u64.into(),
        b: 0u64.into(),
    };

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
            tracing::info!("StarknetError encountered: {}", data.revert_error);
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
            tracing::info!("Failed to deploy contract: {:?}", e);
            Err(RunnerError::DeploymentFailure(
                "Failed to deploy contract".to_string(),
            ))
        }
    }
}

pub fn get_wait_config(wait: bool, retry_interval: u8) -> WaitForTx {
    let waiter_params = ValidatedWaitParams::new(retry_interval, 60);
    WaitForTx {
        wait,
        wait_params: waiter_params,
    }
}
