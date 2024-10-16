use dotenv::dotenv;
use gtw_rs_sdk::GtwSDK;
use std::env;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let bearer_token = env::var("BEARER_TOKEN").expect("BEARER_TOKEN is not set");
    let gtw_sdk = GtwSDK::new(Some(bearer_token))?;

    let wallet_address = "0xa43B0b6dEB096f4dF07a0a122F3Ab824e9118D9D";

    match gtw_sdk.account.wallet().add(wallet_address).await {
        Ok(wallet_create_response) => {
            println!("Wallet Created: {:?}", wallet_create_response);
        }
        Err(e) => {
            eprintln!("Failed to add wallet: {}", e);
        }
    }

    match gtw_sdk.account.wallet().remove(wallet_address).await {
        Ok(remove_response) => {
            println!("Wallet Removed: {:?}", remove_response);
        }
        Err(e) => {
            eprintln!("Failed to remove wallet: {}", e);
        }
    }

    Ok(())
}
