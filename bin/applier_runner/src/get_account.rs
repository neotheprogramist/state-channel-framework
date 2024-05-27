use starknet::{
    accounts::{ExecutionEncoding, SingleOwnerAccount},
    core::types::{BlockId, BlockTag, FieldElement},
    providers::{jsonrpc::HttpTransport, JsonRpcClient},
    signers::{LocalWallet, SigningKey},
};
use url::Url;

pub fn get_account(
    rpc_url: Url,
    chain_id: FieldElement,
    address: FieldElement,
    private_key: FieldElement,
) -> SingleOwnerAccount<JsonRpcClient<HttpTransport>, LocalWallet> {
    let provider = JsonRpcClient::new(HttpTransport::new(rpc_url));

    let signer = LocalWallet::from(SigningKey::from_secret_scalar(private_key));

    let mut account =
        SingleOwnerAccount::new(provider, signer, address, chain_id, ExecutionEncoding::New);
    account.set_block_id(BlockId::Tag(BlockTag::Pending));
    account
}
