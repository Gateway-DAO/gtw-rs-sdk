use dotenv::dotenv;
use gtw_rs_sdk::{
    models::DtoAccountCreateRequest, services::wallet::WalletType, GtwSDK, SdkConfig,
};
use std::env;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenv().ok();

    // Initialize SDK with configuration
    // let bearer_token = env::var("BEARER_TOKEN").expect("BEARER_TOKEN must be set in environment");
    let api_url =
        env::var("API_URL").unwrap_or_else(|_| "https://dev.api.gateway.tech".to_string());

    let sdk_config = SdkConfig {
        api_url: Some(api_url),
        bearer_token: None,
        wallet: Some(WalletType::Ethereum),
        private_key: Some("".to_string()),
    };

    let gtw_sdk = GtwSDK::new(sdk_config).await?;

    // Get account information
    match gtw_sdk.account.get_me().await {
        Ok(account_info) => {
            println!("Account Info: {:?}", account_info);
        }
        Err(e) => {
            eprintln!("Failed to get account info: {}", e);
        }
    }

    // Update account information
    // match gtw_sdk.account.update_me("hhd", "r11manish").await {
    //     Ok(update_account_info) => {
    //         println!("Updated Account Info: {:?}", update_account_info.username);
    //     }
    //     Err(e) => {
    //         eprintln!("Failed to update account info: {}", e);
    //     }
    // }

    // Example of creating a new account
    /*
    let new_account_details = DtoAccountCreateRequest {
        message: "Create Account".to_string(),
        signature: "your_signature".to_string(),
        username: "new_username".to_string(),
        wallet_address: "your_wallet_address".to_string(),
    };

    match gtw_sdk.account.create(new_account_details).await {
        Ok(new_account_info) => {
            println!("Created Account Info: {:?}", new_account_info);
        }
        Err(e) => {
            eprintln!("Failed to create account: {}", e);
        }
    }
    */

    // Get specific account by DID
    // let did =
    //     "did:gatewayid:gateway:d8111b22ac98014ce832f3a6f648d1d6ac0ef833094d2a9bfc26900488acb9ad";
    // match gtw_sdk.account.get_account(did).await {
    //     Ok(account) => {
    //         println!("Account username: {:?}", account.username);
    //     }
    //     Err(e) => {
    //         eprintln!("Failed to retrieve account: {}", e);
    //     }
    // }

    Ok(())
}
