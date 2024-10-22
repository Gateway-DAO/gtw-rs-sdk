use super::wallet::{BlockchainWallet, WalletSignMessage};
use bs58;
use ed25519_dalek::{Keypair, PublicKey, SecretKey, Signature, Signer};
use hex::decode as hex_decode;
use std::error::Error;
use surf::utils::async_trait;

pub struct SolanaWalletService {
    wallet_private_key: Vec<u8>,
    wallet: Keypair,
}

impl SolanaWalletService {
    pub fn new(wallet_private_key: &str) -> Result<Self, Box<dyn Error + Send + Sync>> {
        let keypair = Self::generate_new_solana_wallet_using_private_key(wallet_private_key)?;

        Ok(Self {
            wallet_private_key: keypair.secret.to_bytes().to_vec(),
            wallet: keypair,
        })
    }

    fn generate_new_solana_wallet_using_private_key(
        key: &str,
    ) -> Result<Keypair, Box<dyn Error + Send + Sync>> {
        // Attempt to decode as Base58 first
        if let Ok(private_key_bytes) = bs58::decode(key).into_vec() {
            if private_key_bytes.len() == 64 {
                let secret = SecretKey::from_bytes(&private_key_bytes[..32])?;
                let public = PublicKey::from_bytes(&private_key_bytes[32..])?;
                return Ok(Keypair { secret, public });
            }
        }

        // If Base58 decoding fails, try Hex decoding
        let private_key_bytes = hex_decode(key).map_err(|_| "Invalid hex key format")?;

        if private_key_bytes.len() == 64 {
            let secret = SecretKey::from_bytes(&private_key_bytes[..32])?;
            let public = PublicKey::from_bytes(&private_key_bytes[32..])?;
            return Ok(Keypair { secret, public });
        }

        Err("Invalid private key length: expected 64 bytes for base58 (32 secret + 32 public) or 64 bytes for hex.".into())
    }
}

#[async_trait]
impl BlockchainWallet for SolanaWalletService {
    async fn sign_message(
        &self,
        message: &str,
    ) -> Result<WalletSignMessage, Box<dyn Error + Send + Sync>> {
        let message_bytes = message.as_bytes();
        let signature: Signature = self.wallet.sign(message_bytes);

        let sinaturebs58 = bs58::encode(signature.to_bytes().as_ref()).into_string();

        let pubkey = self.wallet.public;

        let pubkey_base58 = bs58::encode(pubkey.as_ref()).into_string();

        Ok(WalletSignMessage {
            signature: sinaturebs58,
            signing_key: pubkey_base58,
        })
    }
}
