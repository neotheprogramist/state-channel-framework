use crate::models::{AgreeToQuotation, RequestQuotationResponse};
use axum::{
    body::Body,
    http::{Method, Request},
    Router,
};
use serde_json::json;
use serde_json::Value;
use tower::util::ServiceExt;
use dialoguer::console::style;
use server::request::models::SettlementProofResponseWithData;
use server::request::models::Contract;
use server::request::models::RequestQuotationWithPrice;
use server::request::models::SettlementProofResponse;
use server::request::account::MockAccount;
use server::request::account::scalar_to_hex;

#[allow(dead_code)]
pub async fn create_agreement(
    quantity: i64,
    price: i64,
    address: &str,
    url_request_quote: &str,
    url_accept_contract: &str,
    router: Router,
    client_mock_account:MockAccount
) -> Result<(), Box<dyn std::error::Error>> {
    let request_quotation_response =
        request_quote_with_price(address, quantity, url_request_quote, price, router.clone())
            .await?;

    accept_contract(
        request_quotation_response,
        url_accept_contract,
        router.clone(),
        client_mock_account
    )
    .await?;

    Ok(())
}

#[allow(dead_code)]
pub async fn request_settlement_proof_with_price_and_data(
    url: &str,
    address: &String,
    price: i64,
    router: Router,
) -> Result<SettlementProofResponseWithData, Box<dyn std::error::Error>> {
    let url_with_params = format!("{}?address={}&price={}", url, address, price);

    let req = Request::builder()
        .uri(url_with_params)
        .method(axum::http::Method::GET)
        .body(Body::empty())
        .expect("Failed to build request");

    // Send the request using the router
    let response = router
        .oneshot(req)
        .await
        .map_err(|err| Box::new(err) as Box<dyn std::error::Error>)?;

    if !response.status().is_success() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Request failed",
        )));
    }
    let body_bytes: bytes::Bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await?;
    let response_text = String::from_utf8(body_bytes.to_vec())?;

    println!("{}", response_text);

    let json_body: Value = serde_json::from_str(&response_text)?;

    let client_address = json_body["address"]
        .as_str()
        .ok_or("Address not found in JSON response")?
        .to_string();
    let balance: f64 = json_body["balance"]
        .as_f64()
        .ok_or("Balance not found in JSON response or not a valid float")?;
    let diff: i64 = json_body["diff"]
        .as_i64()
        .ok_or("Diff not found in JSON response")?;
    let contracts: Vec<Contract> = serde_json::from_value(json_body["contracts"].clone())
    .map_err(|_| "Failed to parse contracts")?;

    Ok(SettlementProofResponseWithData {
        contracts,
        address: client_address,
        balance,
        diff,
    })
}

#[allow(dead_code)]
pub async fn accept_contract(
    request_quotation_response: RequestQuotationResponse,
    url: &str,
    router: Router,
    client_mock_account: MockAccount
) -> Result<(), Box<dyn std::error::Error>> {
    let data_to_sign = serde_json::to_string(&request_quotation_response)?;
    let quote_data = serde_json::to_string(&data_to_sign).unwrap();
    let quote_bytes = quote_data.as_bytes();

    let mock_account =client_mock_account;
    let client_signature = mock_account.sign_message(quote_bytes);

    let (client_signature_r, client_signature_s) = match client_signature {
        Ok(signature) => (scalar_to_hex(&signature.r), scalar_to_hex(&signature.s)),
        Err(e) => {
            println!("Failed to sign message: {}", e);
            return Err(e.into());
        }
    };

    let request_quotation = AgreeToQuotation {
        quote: request_quotation_response.quote,
        server_signature_r: request_quotation_response.server_signature_r,
        server_signature_s: request_quotation_response.server_signature_s,
        client_signature_r: client_signature_r.to_string(),
        client_signature_s: client_signature_s.to_string(),
    };

    let req = Request::builder()
        .method(Method::POST)
        .uri(url)
        .header("Content-Type", "application/json")
        .body(Body::from(serde_json::to_string(&request_quotation)?))
        .unwrap();
    let agree_to_quotation_response = router.oneshot(req).await?;

    if agree_to_quotation_response.status().is_success() {
        println!("{}", style("Contract created successfully!").green());
        Ok(())
    } else {
        println!(
            "Contract failed with status: {}",
            agree_to_quotation_response.status()
        );
        Err("Contract failed".into())
    }
}

#[allow(dead_code)]
pub async fn request_quote_with_price(
    address: &str,
    quantity: i64,
    url: &str,
    price: i64,
    router: Router,
) -> Result<RequestQuotationResponse, Box<dyn std::error::Error>> {
    let request_quotation = RequestQuotationWithPrice {
        address: address.to_string(),
        quantity,
        price,
    };
    let req = Request::builder()
        .method(Method::POST)
        .uri(url)
        .header("Content-Type", "application/json")
        .body(Body::from(json!(request_quotation).to_string()))
        .unwrap();

    let response = router.oneshot(req).await.unwrap();

    if response.status().is_success() {
        dbg!("SUCCES REQUEST Quote");
        let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await?;
        let response_data: RequestQuotationResponse = serde_json::from_slice(&body_bytes)?;
        Ok(response_data)
    } else {
        dbg!("FAILURE REQUEST");
        let error_message = format!("Failed to get a successful response: {}", response.status());
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            error_message,
        )))
    }
}

#[allow(dead_code)]
async fn request_settlement_proof(
    url: &str,
    address: &String,
    router: Router,
) -> Result<SettlementProofResponse, Box<dyn std::error::Error>> {
    let url_with_params = format!("{}?address={}", url, address);

    let req = Request::builder()
        .uri(url_with_params)
        .method(axum::http::Method::GET)
        .body(Body::empty())
        .expect("Failed to build request");

    // Send the request using the router
    let response = router
        .oneshot(req)
        .await
        .map_err(|err| Box::new(err) as Box<dyn std::error::Error>)?;

    if !response.status().is_success() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Request failed",
        )));
    }
    let body_bytes: bytes::Bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await?;
    let response_text = String::from_utf8(body_bytes.to_vec())?;

    println!("{}", response_text);

    let json_body: Value = serde_json::from_str(&response_text)?;

    let client_address = json_body["address"]
        .as_str()
        .ok_or("Address not found in JSON response")?
        .to_string();
    let balance: f64 = json_body["balance"]
        .as_f64()
        .ok_or("Balance not found in JSON response or not a valid float")?;
    let diff: i64 = json_body["diff"]
        .as_i64()
        .ok_or("Diff not found in JSON response")?;

    Ok(SettlementProofResponse {
        address: client_address,
        balance,
        diff,
    })
}
