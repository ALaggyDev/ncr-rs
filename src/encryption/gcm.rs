use aes::{
    cipher::{typenum::U12, KeyInit},
    Aes128,
};
use aes_gcm::{AeadInPlace, AesGcm};
use rand::Rng;
use std::{convert::Infallible, marker::PhantomData};

use super::Encryption;
use crate::{encoding::Encoding, AesKey, NcrError};

/// The aes/gcm encryption.
#[derive(Debug)]
pub struct GcmEncryption<E: Encoding>(PhantomData<E>);

impl<E: Encoding> GcmEncryption<E> {
    fn raw_encrypt(plaintext: &[u8], key: &AesKey) -> Vec<u8> {
        let mut output = Vec::with_capacity(plaintext.len() + 24);
        let iv = rand::thread_rng().gen::<[u8; 12]>();

        output.extend_from_slice(&iv);
        output.extend_from_slice(plaintext);

        let cipher = AesGcm::<Aes128, U12, U12>::new(key.as_ref().into());

        let tag = cipher
            .encrypt_in_place_detached(&iv.into(), &[], &mut output[12..])
            .unwrap();

        output.extend_from_slice(&tag);

        output
    }

    fn raw_decrypt(ciphertext: Vec<u8>, key: &AesKey) -> Result<String, NcrError> {
        if ciphertext.len() < 24 {
            return Err(NcrError::DecryptError);
        }

        let iv: [u8; 12] = ciphertext[..12].try_into().unwrap();
        let tag: [u8; 12] = ciphertext[(ciphertext.len() - 12)..].try_into().unwrap();

        let mut output = Vec::from(&ciphertext[12..(ciphertext.len() - 12)]);

        let cipher = AesGcm::<Aes128, U12, U12>::new(key.as_ref().into());

        cipher
            .decrypt_in_place_detached(&iv.into(), &[], &mut output, &tag.into())
            .map_err(|_| NcrError::DecryptError)?;

        String::from_utf8(output).map_err(|_| NcrError::DecryptError)
    }
}

impl<E: Encoding> Encryption for GcmEncryption<E> {
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
