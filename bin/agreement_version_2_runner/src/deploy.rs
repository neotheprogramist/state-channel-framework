use starknet::{
    accounts::{AccountError, SingleOwnerAccount},
    contract::ContractFactory,
    core::types::{FieldElement, StarknetError},
    providers::{jsonrpc::HttpTransport, JsonRpcClient, ProviderError},
    signers::LocalWallet,
};

use crate::{
    errors::{parse_contract_address_from_error, RunnerError},
    models::{AgreementConstructor, DeployResult},
};

pub async fn deploy_contract(
    prefunded_account: SingleOwnerAccount<JsonRpcClient<HttpTransport>, LocalWallet>,
    client_public_key: String,
    server_public_key: String,
    class_hash: FieldElement,
    salt: FieldElement,
    udc_address: FieldElement,
) -> Result<DeployResult, RunnerError> {
    let contract_factory: ContractFactory<
        SingleOwnerAccount<JsonRpcClient<HttpTransport>, LocalWallet>,
    > = ContractFactory::new_with_udc(class_hash, prefunded_account, udc_address);
    let agreement_constructor = AgreementConstructor {
        client_balance: 1000000u64.into(),
        server_balance: 1000000u64.into(),
        client_public_key: FieldElement::from_hex_be(&client_public_key)?,
        server_public_key: FieldElement::from_hex_be(&server_public_key)?,
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
    let deployed_address_data = deployment.deployed_address();
    let deploy_result_data = deployment.send().await;
    let transaction_hash = FieldElement::from_hex_be("0x1")?;

    let deployed_address = match deploy_result_data {
        Ok(_) => deployed_address_data,
        Err(AccountError::Provider(ProviderError::StarknetError(
            StarknetError::ContractError(data),
        ))) => {
            if data.revert_error.contains("is unavailable for deployment") {
                parse_contract_address_from_error(&data.revert_error)
            } else {
                return Err(RunnerError::AccountFailure(format!(
                    "Contract error: {}",
                    data.revert_error
                )));
            }
        }
        Err(e) => {
            return Err(RunnerError::AccountFailure(format!("Account error: {}", e)));
        }
    };
    let deploy_result = DeployResult {
        deployed_address,
        transaction_hash,
    };
    Ok(deploy_result)
}
