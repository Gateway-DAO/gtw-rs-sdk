use dotenv::dotenv;
use gtw_rs_sdk::{models::DtoAccountCreateRequest, GtwSDK};
use std::env;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let bearer_token = env::var("BEARER_TOKEN").expect("BEARER_TOKEN is not set");
    let gtw_sdk = GtwSDK::new(Some(bearer_token)).await?;

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
    match gtw_sdk.account.update_me("hhd", "r11manish").await {
        Ok(update_account_info) => {
            println!("Updated Account Info: {:?}", update_account_info.username);
        }
        Err(e) => {
            eprintln!("Failed to update account info: {}", e);
        }
    }

    // let new_account_details = DtoAccountCreateRequest {
    //     message: "Create Account".to_string(),
    //     signature: "your_signature".to_string(),
    //     username: "new_username".to_string(),
    //     wallet_address: "your_wallet_address".to_string(),
    // };

    // match gtw_sdk.account.create(new_account_details).await {
    //     Ok(new_account_info) => {
    //         println!("Created Account Info: {:?}", new_account_info);
    //     }
    //     Err(e) => {
    //         eprintln!("Failed to create account: {}", e);
    //     }
    // }

    let did =
        "did:gatewayid:gateway:d8111b22ac98014ce832f3a6f648d1d6ac0ef833094d2a9bfc26900488acb9ad";
    match gtw_sdk.account.get_account(did).await {
        Ok(account) => {
            println!("Account username: {:?}", account.username);
        }
        Err(e) => {
            eprintln!("Failed to retrieve account: {}", e);
        }
    }

    Ok(())
}
