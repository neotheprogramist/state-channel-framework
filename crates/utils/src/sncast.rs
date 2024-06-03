use serde::{Deserialize, Serialize};
use starknet::core::types::StarknetError::TransactionHashNotFound;
use starknet::providers::Provider;
use starknet::{
    core::types::FieldElement,
    providers::{
        jsonrpc::HttpTransport, JsonRpcClient, ProviderError, ProviderError::StarknetError,
    },
};
use std::{thread::sleep, time::Duration};
use thiserror::Error;
#[derive(Error, Debug)]
pub enum WaitForTransactionError {
    #[error(transparent)]
    TransactionError(TransactionError),
    #[error("sncast timed out while waiting for transaction to succeed")]
    TimedOut,
    #[error("ProviderError")]
    ProviderError,
}
#[derive(Debug, Error)]
pub enum SNCastProviderError {
    #[error(transparent)]
    StarknetError(SNCastStarknetError),
    #[error("Request rate limited")]
    RateLimited,
    #[error("Unknown RPC error: {0}")]
    UnknownError(#[from] anyhow::Error),
}

#[derive(Error, Debug)]
pub enum TransactionError {
    #[error("Transaction has been rejected")]
    Rejected,
    #[error("Transaction has been reverted = {}", .0.data)]
    Reverted(ErrorData),
}
/// More data about the execution failure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct ContractErrorData {
    /// A string encoding the execution trace up to the point of failure
    pub revert_error: String,
}
/// More data about the execution failure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "no_unknown_fields", serde(deny_unknown_fields))]
pub struct TransactionExecutionErrorData {
    /// The index of the first transaction failing in a sequence of given transactions
    pub transaction_index: u64,
    /// A string encoding the execution trace up to the point of failure
    pub execution_error: String,
}

#[derive(Debug, Error)]
pub enum SNCastStarknetError {
    #[error("Node failed to receive transaction")]
    FailedToReceiveTransaction,
    #[error("There is no contract at the specified address")]
    ContractNotFound,
    #[error("Block was not found")]
    BlockNotFound,
    #[error("There is no transaction with such an index")]
    InvalidTransactionIndex,
    #[error("Provided class hash does not exist")]
    ClassHashNotFound,
    #[error("Transaction with provided hash was not found (does not exist)")]
    TransactionHashNotFound,
    #[error("An error occurred in the called contract = {0:?}")]
    ContractError(ContractErrorData),
    #[error("Transaction execution error = {0:?}")]
    TransactionExecutionError(TransactionExecutionErrorData),
    #[error("Contract with the same class hash is already declared")]
    ClassAlreadyDeclared,
    #[error("Invalid transaction nonce")]
    InvalidTransactionNonce,
    #[error("Max fee is smaller than the minimal transaction cost")]
    InsufficientMaxFee,
    #[error("Account balance is too small to cover transaction fee")]
    InsufficientAccountBalance,
    #[error("Contract failed the validation = {0}")]
    ValidationFailure(String),
    #[error("Contract failed to compile in starknet")]
    CompilationFailed,
    #[error("Contract class size is too large")]
    ContractClassSizeIsTooLarge,
    #[error("No account")]
    NonAccount,
    #[error("Transaction already exists")]
    DuplicateTx,
    #[error("Compiled class hash mismatch")]
    CompiledClassHashMismatch,
    #[error("Unsupported transaction version")]
    UnsupportedTxVersion,
    #[error("Unsupported contract class version")]
    UnsupportedContractClassVersion,
    #[error("Unexpected RPC error occurred: {0}")]
    UnexpectedError(anyhow::Error),
}

#[derive(Debug)]
pub struct ErrorData {
    pub data: String,
}

impl ValidatedWaitParams {
    #[must_use]
    pub fn new(retry_interval: u8, timeout: u16) -> Self {
        assert!(
            !(retry_interval == 0 || timeout == 0 || u16::from(retry_interval) > timeout),
            "Invalid values for retry_interval and/or timeout!"
        );

        Self {
            timeout,
            retry_interval,
        }
    }

    #[must_use]
    pub fn get_retries(&self) -> u16 {
        self.timeout / u16::from(self.retry_interval)
    }

    #[must_use]
    pub fn remaining_time(&self, steps_done: u16) -> u16 {
        steps_done * u16::from(self.retry_interval)
    }

    #[must_use]
    pub fn get_retry_interval(&self) -> u8 {
        self.retry_interval
    }

    #[must_use]
    pub fn get_timeout(&self) -> u16 {
        self.timeout
    }
}

pub struct WaitForTx {
    pub wait: bool,
    pub wait_params: ValidatedWaitParams,
}
#[derive(Deserialize, Serialize, Clone, Debug, Copy, PartialEq)]
pub struct ValidatedWaitParams {
    #[serde(default)]
    pub timeout: u16,

    #[serde(
        default,
        rename(serialize = "retry-interval", deserialize = "retry-interval")
    )]
    pub retry_interval: u8,
}

#[allow(dead_code)]
pub const WAIT_TIMEOUT: u16 = 300;
#[allow(dead_code)]
pub const WAIT_RETRY_INTERVAL: u8 = 5;
impl Default for ValidatedWaitParams {
    fn default() -> Self {
        Self::new(WAIT_RETRY_INTERVAL, WAIT_TIMEOUT)
    }
}
pub async fn handle_wait_for_tx<T>(
    provider: &JsonRpcClient<HttpTransport>,
    transaction_hash: FieldElement,
    return_value: T,
    wait_config: WaitForTx,
) -> Result<T, WaitForTransactionError> {
    if wait_config.wait {
        return match wait_for_tx(provider, transaction_hash, wait_config.wait_params).await {
            Ok(_) => Ok(return_value),
            Err(error) => Err(error),
        };
    }

    Ok(return_value)
}

pub async fn wait_for_tx(
    provider: &JsonRpcClient<HttpTransport>,
    tx_hash: FieldElement,
    wait_params: ValidatedWaitParams,
) -> Result<&str, WaitForTransactionError> {
    println!("Transaction hash = {tx_hash:#x}");

    let retries = wait_params.get_retries();
    for i in (1..retries).rev() {
        match provider.get_transaction_status(tx_hash).await {
            Ok(starknet::core::types::TransactionStatus::Rejected) => {
                return Err(WaitForTransactionError::TransactionError(
                    TransactionError::Rejected,
                ));
            }
            Ok(
                starknet::core::types::TransactionStatus::AcceptedOnL2(execution_status)
                | starknet::core::types::TransactionStatus::AcceptedOnL1(execution_status),
            ) => match execution_status {
                starknet::core::types::TransactionExecutionStatus::Succeeded => {
                    return Ok("Transaction accepted")
                }
                starknet::core::types::TransactionExecutionStatus::Reverted => {
                    return get_revert_reason(provider, tx_hash).await
                }
            },
            Ok(starknet::core::types::TransactionStatus::Received)
            | Err(StarknetError(TransactionHashNotFound)) => {
                let remaining_time = wait_params.remaining_time(i);
                println!("Waiting for transaction to be accepted ({i} retries / {remaining_time}s left until timeout)");
            }
            Err(ProviderError::RateLimited) => {
                println!("Request rate limited while waiting for transaction to be accepted");
                sleep(Duration::from_secs(wait_params.get_retry_interval().into()));
            }
            Err(_err) => return Err(WaitForTransactionError::ProviderError),
        };

        sleep(Duration::from_secs(wait_params.get_retry_interval().into()));
    }

    Err(WaitForTransactionError::TimedOut)
}
async fn get_revert_reason(
    provider: &JsonRpcClient<HttpTransport>,
    tx_hash: FieldElement,
) -> Result<&str, WaitForTransactionError> {
    let receipt = provider
        .get_transaction_receipt(tx_hash)
        .await
        .map_err(|_err| WaitForTransactionError::ProviderError)?;

    if let starknet::core::types::ExecutionResult::Reverted { reason } = receipt.execution_result()
    {
        Err(WaitForTransactionError::TransactionError(
            TransactionError::Reverted(ErrorData {
                data: reason.clone(),
            }),
        ))
    } else {
        unreachable!();
    }
}
