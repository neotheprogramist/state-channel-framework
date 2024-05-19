use starknet::{
    accounts::{Account, Call},
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
) {
    let prefunded_account = get_account(rpc_url, chain_id, address, private_key);
    for agreement in agreements {
        let result = prefunded_account
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
            .await
            .unwrap();

        println!("{}", result.transaction_hash);
    }
}
