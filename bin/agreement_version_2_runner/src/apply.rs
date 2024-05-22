use crate::{get_account::get_account, models::FieldElementAgreement};
use starknet::core::types::{PendingTransactionReceipt, StarknetError};
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
    agreements: Vec<FieldElementAgreement>,
    deployed_address: FieldElement,
    rpc_url: Url,
    chain_id: FieldElement,
    address: FieldElement,
    private_key: FieldElement,
) -> Result<FieldElement, Box<dyn std::error::Error>> {
    let prefunded_account = get_account(rpc_url, chain_id, address, private_key);
    let mut gas_fee_sum: FieldElement = FieldElement::from_hex_be("0x0").unwrap();

    for agreement in agreements {
        let send_result = prefunded_account
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
            .fee_estimate_multiplier(2f64)
            .send()
            .await;

        match send_result {
            Ok(result) => {
                println!("Transaction sent: {:?}", result);
                let receipt = wait_for_receipt(&prefunded_account, result.transaction_hash).await?;

                if let Some(overall_fee) = extract_gas_fee(&receipt) {
                    println!("RECEIPT {}", overall_fee);
                    gas_fee_sum = gas_fee_sum + overall_fee;
                } else {
                    eprintln!("Failed to extract gas fee from receipt.");
                    return Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "Failed to extract gas fee from receipt",
                    )));
                }
            }
            Err(err) => {
                eprintln!("Failed to send transaction: {:?}", err);
                return Err(Box::new(err));
            }
        }
    }

    Ok(gas_fee_sum)
}

// Function to extract gas fee from the receipt
fn extract_gas_fee(receipt: &MaybePendingTransactionReceipt) -> Option<FieldElement> {
    println!("extract_gas_fee");

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
        println!("Transaction_hash {:x}", tx_hash);
        match provider.provider().get_transaction_receipt(tx_hash).await {
            Ok(receipt) => {
                println!("Got receipt");
                return Ok(receipt);
            }
            Err(ProviderError::StarknetError(err))
                if err == StarknetError::TransactionHashNotFound && attempts < 10 =>
            {
                attempts += 1;
                sleep(Duration::from_secs(5)).await;
            }
            Err(err) => return Err(err),
        }
    }
}
