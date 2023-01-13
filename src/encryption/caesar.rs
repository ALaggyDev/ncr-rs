use super::Encryption;
use crate::NcrError;

/// The caesar encryption.
///
/// This is a meme cipher. For security reasons, you shouldn't use this encryption.
#[derive(Debug)]
pub struct CaesarEncryption;

const PARAGRAPH: char = '\u{00a7}';
const PARAGRAPH_PLACEHOLDER: char = '\u{ffef}';
const DELETE: char = '\u{007f}';
const DELETE_PLACEHOLDER: char = '\u{fff0}';

impl Encryption for CaesarEncryption {
    type KeyType = u32;
    type EncryptError = NcrError;
    type DecryptError = NcrError;

    fn encrypt(plaintext: &str, key: &u32) -> Result<String, NcrError> {
        let mut output = String::with_capacity(plaintext.len());

        for ch in plaintext.chars() {
            let mut new_ch = char::from_u32(ch as u32 + key).ok_or(NcrError::EncryptError)?;

            if new_ch == PARAGRAPH {
                new_ch = PARAGRAPH_PLACEHOLDER;
            } else if new_ch == DELETE {
                new_ch = DELETE_PLACEHOLDER;
            }

            output.push(new_ch)
        }

        Ok(output)
    }

    fn decrypt(ciphertext: &str, key: &u32) -> Result<String, NcrError> {
        let mut output = String::with_capacity(ciphertext.len());

        for mut ch in ciphertext.chars() {
            if ch == PARAGRAPH_PLACEHOLDER {
                ch = PARAGRAPH;
            } else if ch == DELETE_PLACEHOLDER {
                ch = DELETE;
            }

            let ch_value = u32::checked_sub(ch as u32, *key).ok_or(NcrError::DecryptError)?;

            output.push(char::from_u32(ch_value).ok_or(NcrError::DecryptError)?)
        }

        Ok(output)
    }
}
