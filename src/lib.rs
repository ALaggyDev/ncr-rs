//! # No Chat Reports (NCR) Chat Encryption
//!
//! This crate implements the [No Chat Reports](https://github.com/Aizistral-Studios/No-Chat-Reports)'s custom chat encryption.
//! More specifically this implements a fork of [No Chat Reports](https://github.com/HKS-HNS/No-Chat-Reports).
//!
//! Currently all functionalities of the custom chat encryption are implemented.
//! You can still use this crate normally if you are using the original [No Chat Reports](https://github.com/Aizistral-Studios/No-Chat-Reports).
//!
//! - Caesar, Ecb, Cfb8 and Gcm encryption
//! - Base64 (old), Base64r, [Sus16](https://github.com/HKS-HNS/No-Chat-Reports) and [Mc256](https://github.com/HKS-HNS/No-Chat-Reports) encoding
//! - Passphrase
//!
//! # Examples
//!
//! ## Encrypting
//!
//! ```rust
//! use ncr::{
//!     encoding::Base64rEncoding,
//!     encryption::{Cfb8Encryption, Encryption},
//!     utils::prepend_header,
//!     AesKey,
//! };
//!
//! let key = AesKey::gen_from_passphrase(b"secret");
//!
//! let plaintext = prepend_header("I love Minecraft!");
//! let ciphertext = Cfb8Encryption::<Base64rEncoding>::encrypt(&plaintext, &key).unwrap();
//!
//! println!("{}", ciphertext);
//! ```
//!
//! ## Decrypting
//!
//! ```rust
//! use ncr::{
//!     encoding::Base64rEncoding,
//!     encryption::{Cfb8Encryption, Encryption},
//!     utils::trim_header,
//!     AesKey,
//! };
//!
//! let key = AesKey::gen_from_passphrase(b"secret");
//!
//! let ciphertext = r#"%[2_0»³"!7).«?;!.$¥`¶:8~667ª¸[¬)¢+¤^"#;
//! let plaintext = Cfb8Encryption::<Base64rEncoding>::decrypt(ciphertext, &key).unwrap();
//!
//! let plaintext = trim_header(&plaintext).unwrap();
//!
//! assert_eq!(plaintext, "I love Minecraft!");
//! ```
//!
//! # Features
//!
//! Current there are 4 feature flags.
//!
//!  - `passphrase` (default): Enable key generation from passphrase.
//!  - `cfb8`: Enable aes/cfb8 encryption.
//!  - `ecb`: Enable aes/ecb encryption.
//!  - `gcm`: Enable aes/gcm encryption.
//!

#![cfg_attr(docsrs, feature(doc_auto_cfg))]

mod aes_key;
pub mod encoding;
pub mod encryption;
pub mod utils;

use std::fmt;

pub use aes_key::AesKey;

/// This represents all errors that can happen in this crate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NcrError {
    EncryptError,
    DecryptError,
    DecodeError,
    HeaderError,
}

impl fmt::Display for NcrError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                NcrError::EncryptError => "Encrypt error",
                NcrError::DecryptError => "Decrypt error",
                NcrError::DecodeError => "Decode error",
                NcrError::HeaderError => "Header error",
            }
        )
    }
}
