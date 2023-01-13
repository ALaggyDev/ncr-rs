use aes::{
    cipher::{block_padding::Pkcs7, BlockDecryptMut, BlockEncryptMut, KeyInit},
    Aes128Dec, Aes128Enc,
};
use std::{convert::Infallible, marker::PhantomData};

use super::Encryption;
use crate::{encoding::Encoding, AesKey, NcrError};

/// The aes/ecb encryption.
#[derive(Debug)]
pub struct EcbEncryption<E: Encoding>(PhantomData<E>);

impl<E: Encoding> EcbEncryption<E> {
    fn raw_encrypt(plaintext: &[u8], key: &AesKey) -> Vec<u8> {
        let cipher = Aes128Enc::new(key.as_ref().into());

        // Pkcs5 is a subset of Pkcs7.
        cipher.encrypt_padded_vec_mut::<Pkcs7>(plaintext)
    }

    fn raw_decrypt(ciphertext: Vec<u8>, key: &AesKey) -> Result<String, NcrError> {
        let cipher = Aes128Dec::new(key.as_ref().into());

        // Pkcs5 is a subset of Pkcs7.
        let output = cipher
            .decrypt_padded_vec_mut::<Pkcs7>(&ciphertext)
            .map_err(|_| NcrError::DecryptError)?;

        String::from_utf8(output).map_err(|_| NcrError::DecryptError)
    }
}

impl<E: Encoding> Encryption for EcbEncryption<E> {
    type KeyType = AesKey;
    type EncryptError = Infallible;
    type DecryptError = NcrError;

    fn encrypt(plaintext: &str, key: &AesKey) -> Result<String, Infallible> {
        let ciphertext = Self::raw_encrypt(plaintext.as_bytes(), key);

        Ok(E::encode(&ciphertext))
    }

    fn decrypt(ciphertext: &str, key: &AesKey) -> Result<String, NcrError> {
        let ciphertext = E::decode(ciphertext)?;

        Self::raw_decrypt(ciphertext, key)
    }
}
