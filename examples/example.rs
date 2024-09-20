use dotenv::dotenv;
use gtw_rs_sdk::GtwApi;
use std::env;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let bearer_token = env::var("BEARER_TOKEN").expect("SCHEMA_URL is not set"); //put your token over here
    let gtw_api = GtwApi::new(bearer_token)?;

    match gtw_api.account_info().await {
        Ok(account_info) => {
            println!("Account Info: {:?}", account_info.did.unwrap());
        }
        Err(e) => {
            eprintln!("Failed to get account info: {}", e);
        }
    }

    match gtw_api.update_account_info("hello", "r11manish").await {
        Ok(update_account_info) => {
            println!("Account Info: {:?}", update_account_info.username.unwrap());
        }
        Err(e) => {
            eprintln!("Failed to get account info: {}", e);
        }
    }

    Ok(())
}
