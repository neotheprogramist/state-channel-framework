use crate::{
    deploy::get_wait_config,
    errors::{parse_class_hash_from_error, RunnerError},
};
use sncast::{handle_wait_for_tx, response::errors::StarknetCommandError};
use starknet::{
    accounts::{Account, AccountError, ConnectedAccount, SingleOwnerAccount},
    core::types::{
        contract::{CompiledClass, SierraClass},
        FieldElement, StarknetError,
    },
    providers::{jsonrpc::HttpTransport, JsonRpcClient, ProviderError},
    signers::LocalWallet,
};
use std::sync::Arc;
use tokio::io::AsyncReadExt;

pub async fn declare_contract(
    prefunded_account: SingleOwnerAccount<JsonRpcClient<HttpTransport>, LocalWallet>,
    sierra_path: &str,
    casm_path: &str,
) -> Result<FieldElement, RunnerError> {
    let mut file = tokio::fs::File::open(sierra_path).await?;
    let mut sierra = String::default();
    file.read_to_string(&mut sierra).await?;

    let mut file = tokio::fs::File::open(casm_path).await?;
    let mut casm = String::default();
    file.read_to_string(&mut casm).await?;

    let contract_artifact: SierraClass = serde_json::from_str(&sierra)?;
    let compiled_class: CompiledClass = serde_json::from_str(&casm)?;
    let casm_class_hash = compiled_class.class_hash()?;
    let flattened_class = contract_artifact.clone().flatten()?;

    let result = match prefunded_account
        .declare(Arc::new(flattened_class), casm_class_hash)
        .send()
        .await
    {
        Ok(result) => handle_wait_for_tx(
            prefunded_account.provider(),
            result.transaction_hash,
            result.class_hash,
            get_wait_config(true, 8),
        )
        .await
        .map_err(StarknetCommandError::from),
        Err(AccountError::Provider(ProviderError::StarknetError(
            StarknetError::ContractError(data),
        ))) => {
            tracing::info!("StarknetError encountered: {:?}", data.revert_error);
            if data.revert_error.contains("is already declared") {
                let parsed_class_hash = parse_class_hash_from_error(&data.revert_error);
                tracing::info!("Parsed class hash from error: {:?}", parsed_class_hash);
                Ok(parsed_class_hash)
            } else {
                return Err(RunnerError::AccountFailure(format!(
                    "Contract error: {}",
                    data.revert_error
                )));
            }
        }
        Err(AccountError::Provider(ProviderError::StarknetError(
            StarknetError::TransactionExecutionError(data),
        ))) => {
            tracing::info!(
                "TransactionExecutionError encountered: {:?}",
                data.execution_error
            );
            if data.execution_error.contains("is already declared") {
                let parsed_class_hash = parse_class_hash_from_error(&data.execution_error);
                tracing::info!("Parsed class hash from error: {:?}", parsed_class_hash);
                Ok(parsed_class_hash)
            } else {
                return Err(RunnerError::AccountFailure(format!(
                    "Transaction execution error: {}",
                    data.execution_error
                )));
            }
        }
        Err(e) => {
            tracing::info!("General account error encountered: {:?}", e);
            return Err(RunnerError::AccountFailure(format!("Account error: {}", e)));
        }
    };

    match result {
        Ok(hash) => Ok(hash),
        Err(e) => {
            tracing::info!("Failed to deploy contract: {:?}", e);
            Err(RunnerError::DeploymentFailure(
                "Failed to deploy contract".to_string(),
            ))
        }
    }
}
