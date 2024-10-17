use dotenv::dotenv;
use gtw_rs_sdk::{models::DtoAuthRequest, GtwSDK};
use std::env;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let bearer_token = env::var("BEARER_TOKEN").expect("BEARER_TOKEN is not set");
    let gtw_sdk = GtwSDK::new(Some(bearer_token))?;

    // Attempt to retrieve account information
    match gtw_sdk.account.get_me().await {
        Ok(account_info) => {
            println!("Account Info: {:?}", account_info);
        }
        Err(e) => {
            eprintln!("Failed to get account info: {}", e);
        }
    }

    // Attempt to update account information
    match gtw_sdk.account.update_me("hello", "r11manish").await {
        Ok(update_account_info) => {
            println!("Updated Account Info: {:?}", update_account_info.username);
        }
        Err(e) => {
            eprintln!("Failed to update account info: {}", e);
        }
    }

    // for generate message
    match gtw_sdk.auth.get_message().await {
        Ok(data) => {
            println!("Updated Account Info: {:?}", data.message);
        }
        Err(e) => {
            eprintln!("Failed to update account info: {}", e);
        }
    }

    //  for login

    let login_credetials = DtoAuthRequest {
        message: "type your message".to_string(),
        signature: "type your signature".to_string(),
        wallet_address: "type your address".to_string(),
    };

    // Attempt to log in
    match gtw_sdk.auth.login(login_credetials).await {
        Ok(token_response) => {
            println!("Login successful! Token: {:?}", token_response.token);
        }
        Err(e) => {
            eprintln!("Login failed: {}", e);
        }
    }

    Ok(())
}
