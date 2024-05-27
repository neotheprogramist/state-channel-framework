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

pub const SIERRA_STR: &str =
    include_str!("../../../target/dev/applier_Applier.contract_class.json");
pub const CASM_STR: &str =
    include_str!("../../../target/dev/applier_Applier.compiled_contract_class.json");

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
            println!("Declaration successful, class hash: {:?}", hash.class_hash);
            hash.class_hash
        }
        Err(AccountError::Provider(ProviderError::StarknetError(
            StarknetError::ContractError(data),
        ))) => {
            println!("StarknetError encountered: {:?}", data.revert_error);
            if data.revert_error.contains("is already declared") {
                let parsed_class_hash = parse_class_hash_from_error(&data.revert_error);
                println!("Parsed class hash from error: {:?}", parsed_class_hash);
                parsed_class_hash
            } else {
                return Err(RunnerError::AccountFailure(format!(
                    "Contract error: {}",
                    data.revert_error
                )));
            }
        }
        Err(e) => {
            println!("General account error encountered: {:?}", e);
            return Err(RunnerError::AccountFailure(format!("Account error: {}", e)));
        }
    };
    Ok(class_hash)
}
