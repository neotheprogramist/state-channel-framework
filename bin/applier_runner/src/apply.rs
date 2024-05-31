use crate::account::get_account;
use crate::deploy::get_wait_config;
use crate::models::Agreement;
use sncast::{handle_wait_for_tx, response::errors::StarknetCommandError};
use starknet::core::types::{InvokeTransactionResult, PendingTransactionReceipt, StarknetError};
use starknet::{
    accounts::{Account, Call, ConnectedAccount, SingleOwnerAccount},
    core::types::{FieldElement, MaybePendingTransactionReceipt, TransactionReceipt},
    macros::selector,
    providers::{jsonrpc::HttpTransport, JsonRpcClient, Provider, ProviderError},
    signers::LocalWallet,
};
use std::time::Duration;
use tokio::time::sleep;
use url::Url;
pub async fn apply_agreements(
    agreements: Vec<Agreement>,
    deployed_address: FieldElement,
    rpc_url: Url,
    chain_id: FieldElement,
    address: FieldElement,
    private_key: FieldElement,
) -> Result<FieldElement, Box<dyn std::error::Error>> {
    let prefunded_account = get_account(rpc_url, chain_id, address, private_key);
    let mut gas_fee_sum: FieldElement = FieldElement::from_hex_be("0x0").unwrap();
    let nonce = prefunded_account.get_nonce().await?;

    for (i, agreement) in agreements.iter().enumerate() {
        let fee_estimate_result = prefunded_account
            .execute(vec![Call {
                to: deployed_address,
                selector: selector!("apply"),
                calldata: vec![
                    agreement.quantity,
                    agreement.nonce,
                    agreement.price,
                    agreement.server_signature_r,
                    agreement.server_signature_s,
                    agreement.client_signature_r,
                    agreement.client_signature_s,
                ],
            }])
            .nonce(nonce + i.into())
            .estimate_fee()
            .await;

        let estimated_fee = match fee_estimate_result {
            Ok(fee) => {
                tracing::info!("Estimated Fee for transaction {}: {}", i, fee.overall_fee);
                fee.overall_fee
            }
            Err(e) => {
                tracing::info!("Error estimating fee for transaction {}: {:?}", i, e);
                return Err(Box::new(e));
            }
        };
        tracing::info!("Estimated Fee for transaction {}: {}", i, estimated_fee);

        let adjusted_fee = estimated_fee * 2u64.into();

        let send_result: Result<
            InvokeTransactionResult,
            starknet::accounts::AccountError<
                starknet::accounts::single_owner::SignError<
                    starknet::signers::local_wallet::SignError,
                >,
            >,
        > = prefunded_account
            .execute(vec![Call {
                to: deployed_address,
                selector: selector!("apply"),
                calldata: vec![
                    agreement.quantity,
                    agreement.nonce,
                    agreement.price,
                    agreement.server_signature_r,
                    agreement.server_signature_s,
                    agreement.client_signature_r,
                    agreement.client_signature_s,
                ],
            }])
            .nonce(nonce + i.into())
            .fee_estimate_multiplier(2f64)
            .max_fee(adjusted_fee)
            .send()
            .await;

        let result = match send_result {
            Ok(result) => handle_wait_for_tx(
                prefunded_account.provider(),
                result.transaction_hash,
                InvokeTransactionResult {
                    transaction_hash: result.transaction_hash,
                },
                get_wait_config(true, 1),
            )
            .await
            .map_err(StarknetCommandError::from),

            Err(err) => {
                tracing::info!("Failed to send transaction: {:?}", err);
                return Err(Box::new(err));
            }
        };
        let receipt = wait_for_receipt(&prefunded_account, result?.transaction_hash).await?;
        if let Some(overall_fee) = extract_gas_fee(&receipt) {
            tracing::info!("RECEIPT {}", overall_fee);
            gas_fee_sum += overall_fee;
        } else {
            tracing::info!("Failed to extract gas fee from receipt.");
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to extract gas fee from receipt",
            )));
        }
    }

    Ok(gas_fee_sum)
}

// Function to extract gas fee from the receipt
fn extract_gas_fee(receipt: &MaybePendingTransactionReceipt) -> Option<FieldElement> {
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

// Function to poll for transaction receipt until it's available
async fn wait_for_receipt(
    provider: &SingleOwnerAccount<JsonRpcClient<HttpTransport>, LocalWallet>,
    tx_hash: FieldElement,
) -> Result<MaybePendingTransactionReceipt, ProviderError> {
    let mut attempts = 0;
    loop {
        tracing::info!("Transaction_hash {:x}", tx_hash);
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
