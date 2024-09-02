use dotenv::dotenv;
use gtw_rs_sdk::GTW_API;
use std::env;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let bearer_token = env::var("BEARER_TOKEN").expect("SCHEMA_URL is not set"); //put your token over here
    let gtw_api = GTW_API::new(bearer_token)?;

    match gtw_api.me().await {
        Ok(account_info) => {
            println!("Account Info: {:?}", account_info.did);
        }
        Err(e) => {
            eprintln!("Failed to get account info: {}", e);
        }
    }

    let account_id = 123.0;
    let amount = 50.0;

    match gtw_api.add_funds(account_id, amount).await {
        Ok(ledger_response) => {
            println!("Add Funds Response: {:?}", ledger_response);
        }
        Err(e) => {
            eprintln!("Failed to add funds: {}", e);
        }
    }

    Ok(())
}
