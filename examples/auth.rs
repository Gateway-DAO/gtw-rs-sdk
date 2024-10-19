use dotenv::dotenv;
use gtw_rs_sdk::GtwSDK;
use std::env;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    // Initialize the SDK
    let bearer_token = env::var("BEARER_TOKEN").expect("BEARER_TOKEN is not set");
    let gtw_sdk = GtwSDK::new().await?;

    // Attempt to get a message
    match gtw_sdk.auth.get_message().await {
        Ok(message_response) => {
            println!("Received message: {:?}", message_response.message);
        }
        Err(e) => {
            eprintln!("Failed to get message: {}", e);
        }
    }

    // // Prepare login credentials
    // let login_credentials = DtoAuthRequest {
    //     message: "Enter your message".to_string(),
    //     signature: "Enter your signature".to_string(),
    //     wallet_address: "Enter your wallet address".to_string(),
    // };

    // // Attempt to log in
    // match gtw_sdk.auth.login(login_credentials).await {
    //     Ok(token_response) => {
    //         println!("Login successful! Token: {:?}", token_response.token);
    //     }
    //     Err(e) => {
    //         eprintln!("Login failed: {}", e);
    //     }
    // }

    // Attempt to refresh token
    match gtw_sdk.auth.get_refresh_token().await {
        Ok(token_response) => {
            println!(
                "Token refreshed successfully! New Token: {:?}",
                token_response.token
            );
        }
        Err(e) => {
            eprintln!("Failed to refresh token: {}", e);
        }
    }

    Ok(())
}
