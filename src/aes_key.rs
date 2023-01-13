use base64::{engine::general_purpose::STANDARD, Engine};
use rand::Rng;

use crate::NcrError;

/// Constant salt for all passphrases. [Source](https://github.com/Aizistral-Studios/No-Chat-Reports/blob/87cbf04cbce7db23c23463c889a8864b2c66677d/src/main/java/com/aizistral/nochatreports/encryption/AESEncryption.java#L57-L58)
#[cfg(feature = "passphrase")]
const SALT: [u8; 16] = [
    0x2D, 0x48, 0x18, 0x49, 0x0B, 0x0C, 0x0A, 0x95, 0xFA, 0xA5, 0x44, 0x47, 0x01, 0xD9, 0x99, 0x77,
];

/// Aes key for encryption (128 bits).
#[derive(Clone, PartialEq, Eq, Debug)]
#[repr(transparent)]
pub struct AesKey([u8; 16]);

impl AesKey {
    /// Consumes the key, returns the underlying 128 bits array.
    #[inline]
    pub fn key(self) -> [u8; 16] {
        self.0
    }

    /// Generate a random key.
    pub fn gen_random_key() -> Self {
        Self(rand::thread_rng().gen())
    }

    /// Generate a key from a passphrase.
    #[cfg(feature = "passphrase")]
    pub fn gen_from_passphrase(passphrase: &[u8]) -> Self {
        let mut key = [0u8; 16];
        pbkdf2::pbkdf2::<hmac::Hmac<sha1::Sha1>>(passphrase, &SALT, 65536, &mut key);

        Self(key)
    }

    /// Encode the key as a base64 string.
    pub fn encode_base64(&self) -> String {
        STANDARD.encode(self.0)
    }

    /// Decode the key from a base64 string.
    ///
    /// # Error
    ///
    /// This return a error if the text isn't a base64 string or it's length isn't 128 bits.
    pub fn decode_base64(value: &String) -> Result<Self, NcrError> {
        let result = STANDARD.decode(value).map_err(|_| NcrError::DecodeError)?;

        result
            .as_slice()
            .try_into()
            .map_or_else(|_| Err(NcrError::DecodeError), |value| Ok(Self(value)))
    }
}

impl From<[u8; 16]> for AesKey {
    #[inline]
    fn from(value: [u8; 16]) -> Self {
        Self(value)
    }
}

impl<'a> From<&'a [u8; 16]> for &'a AesKey {
    #[inline]
    fn from(value: &[u8; 16]) -> &AesKey {
        unsafe { &*(value.as_ptr() as *const AesKey) }
    }
}

impl<'a> From<&'a mut [u8; 16]> for &'a mut AesKey {
    #[inline]
    fn from(value: &mut [u8; 16]) -> &mut AesKey {
        unsafe { &mut *(value.as_mut_ptr() as *mut AesKey) }
    }
}

impl AsRef<[u8; 16]> for AesKey {
    #[inline]
    fn as_ref(&self) -> &[u8; 16] {
        &self.0
    }
}
