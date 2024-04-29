
mod test_module {
    use axum::{Json, response::IntoResponse};
    use tower_http::set_header::response;
    use std::env;
    use ed25519_dalek::{ SecretKey, Signature, Signer};
    use serde::{Deserialize, Serialize};
    use crate::request::models::RequestQuotation;
    use crate::request::request_quote::request_quote;
    use crate::server::ServerError;
    #[tokio::test]
    async fn test_request_quote_with_invalid_key() {
        // Set up the environment variable with an invalid key (e.g., too short)
        env::set_var("PRIVATE_KEY", "abcd1234");

        let payload = RequestQuotation {
            address: "test_address".to_string(),
            quantity: 100,
        };

        let response = request_quote(Json(payload)).await;
        match response {
            Ok(result) => println!("Success: {:?}", result),
            Err(e) => println!("Error: {:?}", e),
        }
        assert!(response.is_err());


        env::remove_var("PRIVATE_KEY");
    }
}
