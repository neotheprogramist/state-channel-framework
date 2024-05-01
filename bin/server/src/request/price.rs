use crate::server::ServerError;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct BtcUsdtPriceResponse {
    price: String,
}

pub async fn get_btc_usdt_price() -> Result<f64, ServerError> {
    let response = reqwest::get("https://api.binance.com/api/v3/ticker/price?symbol=BTCUSDT").await;
    match response {
        Ok(res) => {
            if res.status().is_success() {
                let json_response: BtcUsdtPriceResponse =
                    res.json().await.unwrap_or_else(|_| BtcUsdtPriceResponse {
                        price: "0".to_string(),
                    });
                let price_f64: f64 = json_response.price.parse().unwrap_or(0.0);
                Ok(price_f64)
            } else {
                Err(ServerError::BTCRequestFailure(
                    "Failed to fetch or parse price.".to_string(),
                ))
            }
        }
        Err(err) => Err(ServerError::BTCRequestFailure(err.to_string())),
    }
}
