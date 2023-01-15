//! Some common encoding algorithms.
//!
//! # Examples
//!
//! ## Encoding
//!
//! ```
//! use ncr::encoding::{Base64Encoding, Encoding};
//!
//! let decoded = "#%Hello, world!";
//! let encoded = Base64Encoding::encode(decoded.as_bytes());
//!
//! assert_eq!(encoded, "IyVIZWxsbywgd29ybGQh");
//! ```
//!
//! ## Decoding
//!
//! ```
//! use ncr::encoding::{Base64Encoding, Encoding};
//!
//! let encoded = "IyVIZWxsbywgd29ybGQh";
//! let decoded = Base64Encoding::decode(encoded).unwrap();
//!
//! assert_eq!(String::from_utf8(decoded).unwrap(), "#%Hello, world!");
//! ```
//!

mod base64;
mod base64r;
mod mc256;
mod sus16;

use crate::NcrError;

pub use self::base64::Base64Encoding;
pub use base64r::{Base64rEncoding, NewBase64rEncoding};
pub use mc256::Mc256Encoding;
pub use sus16::Sus16Encoding;

/// The encoding trait.
pub trait Encoding {
    /// Encode a given text.
    fn encode(text: &[u8]) -> String;

    /// Decode a given text.
    fn decode(text: &str) -> Result<Vec<u8>, NcrError>;
}
