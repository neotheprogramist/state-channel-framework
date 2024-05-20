use starknet::{
    accounts::{Account, Call, ConnectedAccount},
    core::types::FieldElement,
    macros::selector,
};
use url::Url;

use crate::{get_account::get_account, models::FieldElementAgreement};

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

    // let mut current_nonce = prefunded_account.get_nonce().await.unwrap() ;
    for agreement in agreements {
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
            .estimate_fee()
            .await;
        match fee_estimate_result {
            Ok(fee_estimate) => {
                gas_fee_sum += fee_estimate.overall_fee;
            }
            Err(err) => {
                eprintln!("Failed to estimate fee: {:?}", err);
                return Err(Box::new(err));
            }
        }

        // Send the transaction
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
            .send()
            .await;

        match send_result {
            Ok(result) => {
                println!("Transaction sent: {:?}", result);
            }
            Err(err) => {
                eprintln!("Failed to send transaction: {:?}", err);
                return Err(Box::new(err));
            }
        }
    }
    Ok(gas_fee_sum)
}
