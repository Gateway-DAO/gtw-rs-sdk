use std::error::Error;
use surf::utils::async_trait;

use crate::WalletType;

use super::{ethereum::EthereumService, solana::SolanaWalletService};

#[async_trait]
pub trait BlockchainWallet {
    async fn sign_message(
        &self,
        message: &str,
    ) -> Result<WalletSignMessage, Box<dyn Error + Send + Sync>>;
}

#[derive(Clone, Debug)]


pub struct WalletSignMessage {
    pub signature: String,
    pub signing_key: String,
}

pub struct WalletService {
    wallet: Box<dyn BlockchainWallet + Send + Sync>,
}

impl WalletService {
    pub fn new(
        wallet_private_key: String,
        wallet_type: Option<WalletType>,
    ) -> Result<Self, Box<dyn Error + Send + Sync>> {
        let wallet: Box<dyn BlockchainWallet + Send + Sync> =
            match wallet_type.unwrap_or(WalletType::Ethereum) {
                WalletType::Ethereum => Box::new(EthereumService::new(&wallet_private_key)?),
                // WalletType::Sui => Box::new(SuiWalletService::new(&wallet_private_key)?),
                WalletType::Solana => Box::new(SolanaWalletService::new(&wallet_private_key)?),
            };

        Ok(WalletService { wallet })
    }

    pub async fn sign_message(
        &self,
        data: &str,
    ) -> Result<WalletSignMessage, Box<dyn Error + Send + Sync>> {
        self.wallet.sign_message(data).await
    }
}
