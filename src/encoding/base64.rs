use base64::{
    alphabet::Alphabet,
    engine::{general_purpose::PAD, GeneralPurpose},
    Engine,
};

use super::Encoding;
use crate::NcrError;

/// The base64 encoding.
///
/// # Important
///
/// The character "/" is replaced with "\\" to prevent Minecraft from recognizing this as a command.
/// See [No Chat Reports](https://github.com/HKS-HNS/No-Chat-Reports/blob/9088c8501e4259325476d0c5a751a368b96036d3/src/main/java/com/aizistral/nochatreports/encryption/Encryptor.java#L93).
#[derive(Debug)]
pub struct Base64Encoding;

impl Encoding for Base64Encoding {
    fn encode(text: &[u8]) -> String {
        BASE64_ENGINE.encode(text)
    }

    fn decode(text: &str) -> Result<Vec<u8>, NcrError> {
        BASE64_ENGINE
            .decode(text)
            .map_err(|_| NcrError::DecodeError)
    }
}

const BASE64_ALPHABET: Alphabet =
    match Alphabet::new("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+\\") {
        Ok(alphabet) => alphabet,
        Err(_) => panic!(),
    };
const BASE64_ENGINE: GeneralPurpose = GeneralPurpose::new(&BASE64_ALPHABET, PAD);
