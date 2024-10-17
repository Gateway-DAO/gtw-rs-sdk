use ethers::prelude::*;
use ethers::signers::LocalWallet;
use ethers::utils::keccak256;
use hex::encode as hex_encode;
use std::error::Error;

pub struct EthereumService {
    wallet_private_key: String,
    wallet: LocalWallet,
}

pub struct WalletSignMessageType {
    pub signature: String,
    pub signing_key: Address,
}

impl EthereumService {
    pub fn new(wallet_private_key: &str) -> Result<Self, Box<dyn Error>> {
        let wallet = Self::generate_new_ethereum_wallet_using_private_key(wallet_private_key)?;

        Ok(Self {
            wallet_private_key: wallet_private_key.to_string(),
            wallet,
        })
    }

    pub fn generate_new_ethereum_wallet_using_private_key(
        hex_private_key: &str,
    ) -> Result<LocalWallet, Box<dyn Error>> {
        let private_key_bytes = hex::decode(hex_private_key)?;
        let wallet = LocalWallet::from_bytes(&private_key_bytes)?;
        Ok(wallet)
    }

    pub fn sign_message(&self, message: &str) -> Result<WalletSignMessageType, Box<dyn Error>> {
        let message_hash = keccak256(format!(
            "\x19Ethereum Signed Message:\n{}{}",
            message.len(),
            message
        ));

        let signature = self.wallet.sign_hash(H256::from(message_hash))?;

        let mut signature_bytes = signature.to_vec();

        if signature_bytes[64] < 27 {
            signature_bytes[64] += 27;
        }

        Ok(WalletSignMessageType {
            signature: hex_encode(signature_bytes),
            signing_key: self.wallet.address(),
        })
    }
}
