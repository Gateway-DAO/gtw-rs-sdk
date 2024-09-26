use dotenv::dotenv;
use gtw_rs_sdk::GtwSDK;
use std::env;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let bearer_token = env::var("BEARER_TOKEN").expect("SCHEMA_URL is not set"); //put your token over here
    let gtw_sdk = GtwSDK::new(Some(bearer_token))?;

    match gtw_sdk.account_info().await {
        Ok(account_info) => {
            println!("Account Info: {:?}", account_info.did);
        }
        Err(e) => {
            eprintln!("Failed to get account info: {}", e);
        }
    }

    match gtw_sdk.update_account_info("hello", "r11manish").await {
        Ok(update_account_info) => {
            println!(" Update Account Info: {:?}", update_account_info.username);
        }
        Err(e) => {
            eprintln!("Failed to get account info: {}", e);
        }
    }

    // match gtw_sdk.generate_message().await {
    //     Ok(generate_message) => {
    //         println!("Message Generated: {:?}", generate_message.message);
    //     }
    //     Err(e) => {
    //         eprintln!("Failed to get account info: {}", e);
    //     }
    // }

    Ok(())
}
