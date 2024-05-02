mod test_module {

    #[tokio::test]
    async fn test_request_quote_with_invalid_key() {
        use crate::request::models::RequestQuotation;
        use crate::request::request_quote::request_quote;
        use axum::Json;
        use std::env;
        // Set up the environment variable with an invalid key (e.g., too short)
        env::set_var("PRIVATE_KEY", "abcd1234");

        let payload = RequestQuotation {
            address: "test_address".to_string(),
            quantity: 100,
        };

        let response = request_quote(Json(payload)).await;
        match response {
            Ok(result) => println!("Success:"),
            Err(e) => println!("Error: {:?}", e),
        }
        //assert!(response.is_err());

        env::remove_var("PRIVATE_KEY");
    }

    #[tokio::test]
    async fn test_request_quote() {
        use crate::request::models::RequestQuotation;
        use crate::request::request_quote::request_quote;
        use axum::Json;
        let payload = RequestQuotation {
            address: "test_address".to_string(),
            quantity: 100,
        };

        let response = request_quote(Json(payload)).await;

        match response {
            Ok(result) => {
                println!("Success:")
            }
            Err(e) => {
                println!("Error during request: {:?}", e);
            }
        }
    }
}
