use std::time::Duration;

use starknet::accounts::{Account, Call, ConnectedAccount, SingleOwnerAccount};
use starknet::core::types::{FieldElement, TransactionExecutionStatus, TransactionStatus};
use starknet::core::utils::get_selector_from_name;
use starknet::providers::jsonrpc::HttpTransport;
use starknet::providers::{JsonRpcClient, Provider};
use starknet::signers::LocalWallet;
use tokio::time::sleep;

pub async fn invoke(
    account: &SingleOwnerAccount<JsonRpcClient<HttpTransport>, LocalWallet>,
    contract_address: FieldElement,
    selector_name: &str,
    calldata: Vec<FieldElement>,
) -> anyhow::Result<FieldElement> {
    let tx = account
        .execute(vec![Call {
            to: contract_address,
            selector: get_selector_from_name(selector_name).unwrap(),
            calldata,
        }])
        .send()
        .await
        .expect("Failed to send `verify_and_register_fact` transaction.");

    let start_fetching = std::time::Instant::now();
    let wait_for = Duration::from_secs(60);
    let execution_status = loop {
        if start_fetching.elapsed() > wait_for {
            anyhow::bail!("Transaction not mined in {} seconds.", wait_for.as_secs());
        }

        let status = match account
            .provider()
            .get_transaction_status(tx.transaction_hash)
            .await
        {
            Ok(status) => status,
            Err(_e) => {
                sleep(Duration::from_secs(1)).await;
                continue;
            }
        };

        break match status {
            TransactionStatus::Received => {
                println!("Transaction received.");
                sleep(Duration::from_secs(1)).await;
                continue;
            }
            TransactionStatus::Rejected => {
                anyhow::bail!("Transaction {:#x} rejected.", tx.transaction_hash);
            }
            TransactionStatus::AcceptedOnL2(execution_status) => execution_status,
            TransactionStatus::AcceptedOnL1(execution_status) => execution_status,
        };
    };

    match execution_status {
        TransactionExecutionStatus::Succeeded => {
            println!("Transaction accepted on L2.");
        }
        TransactionExecutionStatus::Reverted => {
            anyhow::bail!("Transaction failed with.");
        }
    }

    Ok(tx.transaction_hash)
}
