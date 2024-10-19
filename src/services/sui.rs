use base58::ToBase58;
use ed25519_dalek::{Keypair, PublicKey, SecretKey, Signature, Signer};
use hex::encode as hex_encode;
use std::error::Error;
use surf::utils::async_trait;

use super::wallet::{BlockchainWallet, WalletSignMessage};

pub struct SuiWalletService {
    keypair: Keypair,
    address: String,
}

impl SuiWalletService {
    pub fn new(private_key: &str) -> Result<Self, Box<dyn Error + Send + Sync>> {
        let secret_bytes = hex::decode(private_key)?;
        let secret_key = SecretKey::from_bytes(&secret_bytes)?;

        let public_key: PublicKey = (&secret_key).into();

        let keypair = Keypair {
            secret: secret_key,
            public: public_key,
        };

        let address = public_key.as_bytes().to_base58();

        Ok(Self { keypair, address })
    }

    pub fn wallet_address_to_string(&self) -> String {
        self.address.clone()
    }
}

#[async_trait]
impl BlockchainWallet for SuiWalletService {
    async fn sign_message(
        &self,
        message: &str,
    ) -> Result<WalletSignMessage, Box<dyn Error + Send + Sync>> {
        let signature: Signature = self.keypair.sign(message.as_bytes());
        let signature_hex = hex_encode(signature.to_bytes());
        let address_hex = self.wallet_address_to_string();

        Ok(WalletSignMessage {
            signature: signature_hex,
            signing_key: address_hex,
        })
    }
}
