//! gemblockchain
//!
//! Library to work with Gem blockchain
//!
//! # Usage
//! ```
//! use gemblockchain::GemAddress;
//! let gem = GemAddress::generate(None);
//! println!("Address: {}", gem.address);
//! println!("Mnemonic phrase: {}", gem.mnemonic_phrase);
//! println!("Mini secret key: {}", gem.mini_secret_key_to_string());
//! println!("Public key: {}", gem.public_key_to_string());
//! ```
use data_encoding::HEXLOWER;
use sp_core::crypto::AccountId32;
use sp_core::crypto::Ss58AddressFormat;
use sp_core::crypto::Ss58Codec;
use sp_core::Pair;

/// Address format
pub const SS58FORMAT: u8 = 44 as u8;

/// Gem address struct
pub struct GemAddress {
    pub mnemonic_phrase: String,
    pub mini_secret_key: [u8; 32],
    pub public_key: [u8; 32],
    pub address: String,
}

impl GemAddress {
    /// Generate Gem address
    pub fn generate(password: Option<&str>) -> GemAddress {
        let (pair, phrase, secret) = sp_core::sr25519::Pair::generate_with_phrase(password);
        let address = AccountId32::from(pair.public())
            .to_ss58check_with_version(Ss58AddressFormat::Custom(SS58FORMAT));
        GemAddress {
            mnemonic_phrase: phrase,
            // mini_secret_key: HEXLOWER.encode(&secret),
            mini_secret_key: secret,
            public_key: <[u8; 32]>::from(pair.public()),
            // public_key: HEXLOWER.encode(&<[u8; 32]>::from(pair.public())),
            address,
        }
    }

    /// Returns the secret key in string format
    pub fn mini_secret_key_to_string(&self) -> String {
        HEXLOWER.encode(&self.mini_secret_key)
    }

    /// Returns the public key in string format
    pub fn public_key_to_string(&self) -> String {
        HEXLOWER.encode(&self.public_key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_address() {
        let address = GemAddress::generate(None);

        assert_eq!(address.mnemonic_phrase.split_ascii_whitespace().count(), 12);
    }
}
