use serde::Deserialize;
use crate::server::ServerError;
#[derive(Debug, Deserialize)]
struct BtcUsdtPriceResponse {
    price: String,
}

pub async fn get_btc_usdt_price() -> Result<u64, ServerError> {
    // Send GET request to Binance API
    let response = match reqwest::get("https://api.binance.com/api/v3/ticker/price?symbol=BTCUSDT").await {
        Ok(response) => {
            // Deserialize JSON response
            let json_response: BtcUsdtPriceResponse = response.json().await?;
            // Parse price string to float
            let price: f64 = json_response.price.parse().unwrap_or(0.0);

            // Convert price to u64
            let price_u64 = price as u64;

            // Return price as Result<u64, Error>
            
            return Ok(price_u64)
        },
        Err(err) => {
            return Err(ServerError::BTCRequestFailure); // Convert reqwest::Error to string
        }
    };

}