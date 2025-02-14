use reqwest::blocking::Client;
use serde_json::json;

pub fn broadcast_transaction(raw_tx: &[u8], infura_url: &str) {
    let client = Client::new();
    let params = json!([format!("0x{}", hex::encode(raw_tx))]);
    let body = json!({
        "jsonrpc": "2.0",
        "method": "eth_sendRawTransaction",
        "params": params,
        "id": 1,
    });

    match client.post(infura_url).json(&body).send() {
        Ok(response) => {
            println!("Response: {:?}", response.text().unwrap());
        }
        Err(err) => {
            println!("Failed to broadcast transaction: {}", err);
        }
    }
}
