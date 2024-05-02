use super::models::JWTResponse;
use super::ValidatorErrors;
use ed25519_dalek::SigningKey;
use ed25519_dalek::VerifyingKey;
use ed25519_dalek::{Signature, Signer};
use reqwest::cookie::Jar;
use reqwest::Client;
use reqwest::Url;
use serde_json::json;
use serde_json::Value;
use std::sync::Arc;

pub struct ProverSDK {
    client: Client,
}

impl ProverSDK {
    pub fn new() -> Self {
        ProverSDK {
            client: Client::new(),
        }
    }

    pub fn with_cookies(jwt_token: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let cookie = format!("jwt_token={}; HttpOnly; Secure; Path=/", jwt_token);
        let url = "http://localhost:7003/prove/state-diff-commitment".parse::<Url>()?;

        let jar = Jar::default();
        jar.add_cookie_str(&cookie, &url);

        let client = reqwest::Client::builder()
            .cookie_provider(Arc::new(jar))
            .build()?;

        Ok(ProverSDK { client })
    }

    pub async fn authenticate_with_prover(
        &self,
        signing_key: &SigningKey,
        public_key: &VerifyingKey,
        url: &String,
    ) -> Result<JWTResponse, ValidatorErrors> {
        // (Prover authentication) call get nonce
        let nonce = self.get_nonce(&public_key, url, &self.client).await?;
        println!("Got Nonce: {}", nonce);

        // Sign the nonce
        let signed_nonce = signing_key.sign(nonce.as_bytes()); //slice string to bytes

        // Validate our signature and get Jwt_token
        let jwt_token = self
            .validate_signature(&public_key, &nonce, &signed_nonce, url, &self.client)
            .await?;
        Ok(jwt_token)
    }

    async fn get_nonce(
        &self,
        public_key: &VerifyingKey,
        url: &String,
        client: &Client,
    ) -> Result<String, ValidatorErrors> {
        let url_with_params = format!(
            "{}?public_key={}",
            url,
            bytes_to_hex_string(public_key.as_bytes())
        );

        let response = match client.get(url_with_params).send().await {
            Ok(response) => response,
            Err(err) => return Err(ValidatorErrors::RequestFailed(err)),
        };

        let response_text = match response.text().await {
            Ok(text) => text,
            Err(err) => return Err(ValidatorErrors::RequestFailed(err)),
        };
        let json_body: Value = match serde_json::from_str(&response_text) {
            Ok(json) => json,
            Err(_err) => return Err(ValidatorErrors::JsonParsingFailed),
        };
        let nonce = match json_body["nonce"].as_str() {
            Some(nonce) => nonce.to_string(),
            None => return Err(ValidatorErrors::NonceNotFound),
        };
        Ok(nonce)
    }

    async fn validate_signature(
        &self,
        public_key: &VerifyingKey,
        nonce: &String,
        signed_nonce: &Signature,
        url: &String,
        client: &Client,
    ) -> Result<JWTResponse, ValidatorErrors> {
        let data = json!({
            "public_key": bytes_to_hex_string(&public_key.to_bytes()),
            "nonce": nonce,
            "signature":bytes_to_hex_string(&signed_nonce.to_bytes()),
        });

        // Send Post request to validation
        let response = client
            .post(url)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .json(&data)
            .send()
            .await?;

        let json_body: Value = match response.json().await {
            Ok(json) => json,
            Err(_e) => return Err(ValidatorErrors::JsonParsingFailed),
        };
        let jwt_token = match json_body["jwt_token"].as_str() {
            Some(jwt_token) => jwt_token.to_string(),
            None => return Err(ValidatorErrors::NonceNotFound),
        };
        let expiration = match json_body["expiration"].as_u64() {
            Some(expiration) => expiration,
            None => return Err(ValidatorErrors::NonceNotFound),
        };
        Ok(JWTResponse {
            jwt_token,
            expiration,
        })
    }

    pub async fn proof(&self, data: Value, url: &str) -> Result<String, ValidatorErrors> {
        let response = self.client.post(url).json(&data).send().await?;

        let response_data = response.text().await?;
        println!("{}", response_data);

        Ok(response_data)
    }
}

// Convert byte array to hexadecimal string
fn bytes_to_hex_string(bytes: &[u8]) -> String {
    hex::encode(bytes)
}
