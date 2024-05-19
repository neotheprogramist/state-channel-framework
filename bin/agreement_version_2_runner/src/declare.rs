use crate::errors::{parse_class_hash_from_error, RunnerError};
use starknet::{
    accounts::{Account, AccountError, SingleOwnerAccount},
    core::types::{
        contract::{CompiledClass, SierraClass},
        FieldElement, StarknetError,
    },
    providers::{Provider, ProviderError},
    signers::Signer,
};
use std::sync::Arc;

pub const SIERRA_STR: &str = include_str!("../../../src/agreement_version_2/target/dev/agreement_version_2_AgreementVersion2.contract_class.json");
// We can store only the class_hash and thus te casm_str would not be needed but for now it is
pub const CASM_STR: &str = include_str!("../../../src/agreement_version_2/target/dev/agreement_version_2_AgreementVersion2.compiled_contract_class.json");

pub async fn declare_contract<P, S>(
    prefunded_account: &SingleOwnerAccount<P, S>,
) -> Result<FieldElement, RunnerError>
where
    P: Provider + Send + Sync,
    S: Signer + Send + Sync,
{
    let contract_artifact: SierraClass = serde_json::from_str(SIERRA_STR)?;
    let compiled_class: CompiledClass = serde_json::from_str(CASM_STR)?;
    let casm_class_hash = compiled_class.class_hash()?;
    let flattened_class = contract_artifact.clone().flatten()?;

    let result = prefunded_account
        .declare(Arc::new(flattened_class), casm_class_hash)
        .send()
        .await;

    let class_hash = match result {
        Ok(hash) => {
            // If the contract is successfully declared, use this hash
            hash.class_hash
        }
        Err(AccountError::Provider(ProviderError::StarknetError(
            StarknetError::ContractError(data),
        ))) => {
            if data.revert_error.contains("is already declared") {
                parse_class_hash_from_error(&data.revert_error)
            } else {
                return Err(RunnerError::AccountError(format!(
                    "Contract error: {}",
                    data.revert_error
                )));
            }
        }
        Err(e) => {
            return Err(RunnerError::AccountError(format!(
                "Account error: {}",
                e.to_string()
            )));
        }
    };

    Ok(class_hash)
}
