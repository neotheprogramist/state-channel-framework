use starknet::core::types::{PendingTransactionReceipt, StarknetError, TransactionReceipt};
use starknet::{
    accounts::{ConnectedAccount, SingleOwnerAccount},
    core::types::{FieldElement, MaybePendingTransactionReceipt},
    providers::{jsonrpc::HttpTransport, JsonRpcClient, Provider, ProviderError},
    signers::LocalWallet,
};
use std::time::Duration;
use tokio::time::sleep;

// Function to poll for transaction receipt until it's available
pub async fn wait_for_receipt(
    provider: &SingleOwnerAccount<JsonRpcClient<HttpTransport>, LocalWallet>,
    tx_hash: FieldElement,
) -> Result<MaybePendingTransactionReceipt, ProviderError> {
    let mut attempts = 0;
    loop {
        println!("Transaction_hash {:x}", tx_hash);
        match provider.provider().get_transaction_receipt(tx_hash).await {
            Ok(receipt) => {
                return Ok(receipt);
            }
            Err(ProviderError::StarknetError(err))
                if err == StarknetError::TransactionHashNotFound && attempts < 20 =>
            {
                attempts += 1;
                sleep(Duration::from_secs(5)).await;
            }
            Err(err) => return Err(err),
        }
    }
}

// Function to extract gas fee from the receipt
pub fn extract_gas_fee(receipt: &MaybePendingTransactionReceipt) -> Option<FieldElement> {
    match receipt {
        MaybePendingTransactionReceipt::Receipt(receipt) => match receipt {
            TransactionReceipt::Invoke(receipt) => Some(receipt.actual_fee.amount),
            TransactionReceipt::L1Handler(receipt) => Some(receipt.actual_fee.amount),
            TransactionReceipt::Declare(receipt) => Some(receipt.actual_fee.amount),
            TransactionReceipt::Deploy(receipt) => Some(receipt.actual_fee.amount),
            TransactionReceipt::DeployAccount(receipt) => Some(receipt.actual_fee.amount),
        },
        MaybePendingTransactionReceipt::PendingReceipt(receipt) => match receipt {
            PendingTransactionReceipt::Invoke(receipt) => Some(receipt.actual_fee.amount),
            PendingTransactionReceipt::L1Handler(receipt) => Some(receipt.actual_fee.amount),
            PendingTransactionReceipt::Declare(receipt) => Some(receipt.actual_fee.amount),
            PendingTransactionReceipt::DeployAccount(receipt) => Some(receipt.actual_fee.amount),
        },
    }
}
