use serde_json::Value;
use reqwest;


pub async fn eth_call(request_body: Value) -> Result<Value, reqwest::Error> {
    let client = reqwest::Client::new();
    let response = client.post("https://eth.drpc.org").json(&request_body).send().await?;
    let response_json: Value = response.json().await?;
    Ok(response_json)
}