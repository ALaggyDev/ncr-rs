//! Some common encryption algorithms.
//!
//! # Examples
//!
//! ## Encrypting
//!
//! ```
//! use ncr::encryption::{CaesarEncryption, Encryption};
//!
//! let decrypted = "#%Hello, world!";
//! let encrypted = CaesarEncryption::encrypt(decrypted, &5).unwrap();
//!
//! assert_eq!(encrypted, "(*Mjqqt1%|twqi&");
//! ```
//!
//! ## Decrypting
//!
//! ```
//! use ncr::encryption::{CaesarEncryption, Encryption};
//!
//! let encrypted = "(*Mjqqt1%|twqi&";
//! let decrypted = CaesarEncryption::decrypt(encrypted, &5).unwrap();
//!
//! assert_eq!(decrypted, "#%Hello, world!");
//! ```
//!

mod caesar;
#[cfg(feature = "cfb8")]
mod cfb8;
#[cfg(feature = "ecb")]
mod ecb;
#[cfg(feature = "gcm")]
mod gcm;

pub use self::caesar::CaesarEncryption;
#[cfg(feature = "cfb8")]
pub use self::cfb8::Cfb8Encryption;
#[cfg(feature = "ecb")]
pub use self::ecb::EcbEncryption;
#[cfg(feature = "gcm")]
pub use self::gcm::GcmEncryption;

/// The encryption trait.
pub trait Encryption {
    type KeyType;
    type EncryptError;
    type DecryptError;

    /// Encrypt a given text.
    fn encrypt(plaintext: &str, key: &Self::KeyType) -> Result<String, Self::EncryptError>;

    /// Decrypt a given text.
    fn decrypt(ciphertext: &str, key: &Self::KeyType) -> Result<String, Self::DecryptError>;
}
