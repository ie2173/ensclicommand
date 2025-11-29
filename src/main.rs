mod keccak256;
mod name_hasher;
mod eth_call;


use clap::Parser;
use ens_normalize_rs::normalize;
use crate::eth_call::eth_call;
use name_hasher::namehash;


#[derive(Parser)]
#[command(name = "ensure")]
#[command(version="0.1.0")]
#[command(about = "A simple cli tool to get a proper ethereum address from an ens name", long_about = None)]
struct Cli {
     #[arg(value_name = "ENS_NAME")]
    name: String,
}

pub fn extract_address_from_response(response: &str) -> Option<String> {
    if response.len() < 66 {
        return None;
    }
    Some(format!("0x{}", &response[26..66]))
}


const RESOLVER_FUNCTION_SELECTOR: [u8; 4] = [0x01, 0x78, 0xb8, 0xbf];
const ADDR_FUNCTION_SELECTOR: [u8; 4] = [0x3b, 0x3b, 0x57, 0xde];

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let normalized_name = normalize(&cli.name).unwrap_or_else(|e| {
        eprintln!("Error normalizing name: {}", e);
        std::process::exit(1);
    });

    let ens_node = namehash(&normalized_name);

    // create client to connect to ethereum node

    // first call general contract for resolver
    let resolver_request_body = serde_json::json!({
        "jsonrpc":"2.0",
        "method": "eth_call",
        "params":[{
            "to": "0x00000000000C2E074eC69A0dFb2997BA6C7d2e1e",
            "data": format!("0x{}{}", hex::encode(RESOLVER_FUNCTION_SELECTOR), hex::encode(ens_node))
        },"latest"],
        "id":1
    });

   let resolver_response = eth_call(resolver_request_body).await?;
   let results_hex = resolver_response["result"].as_str().unwrap_or("0x0");
   let address = extract_address_from_response(results_hex);

    // call resolver for address
     let addr_request_body = serde_json::json!({
        "jsonrpc":"2.0",
        "method": "eth_call",
        "params":[{
            "to": format!("{}", (address.unwrap_or_default())),
            "data": format!("0x{}{}", hex::encode(ADDR_FUNCTION_SELECTOR), hex::encode(ens_node))
        },"latest"],
        "id":1
    });
    let addr_response = eth_call(addr_request_body).await?;
    let addr_results_hex = addr_response["result"].as_str().unwrap_or("0x0");
    let string_adress = extract_address_from_response(addr_results_hex).unwrap();
    println!("{}", string_adress);
    Ok(())
}
