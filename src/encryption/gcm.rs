use aes::cipher::KeyInit;
use aes_gcm::{AeadInPlace, Aes128Gcm};
use rand::Rng;
use std::{convert::Infallible, marker::PhantomData};
use subtle::ConstantTimeEq;

use super::Encryption;
use crate::{encoding::Encoding, AesKey, NcrError};

/// The aes/gcm encryption.
#[derive(Debug)]
pub struct GcmEncryption<E: Encoding>(PhantomData<E>);

impl<E: Encoding> GcmEncryption<E> {
    // TODO: Gcm encryption is the weirdest encryption in here.
    // NCR uses a 12 byte authorization tag, but most implementation uses 16 byte authorization tag instead.
    // I have made a pull request (merged) on the Aead github repository:
    // https://github.com/RustCrypto/AEADs/pull/501
    // Now we are just waiting for aes-gcm crate to update.

    fn raw_encrypt(plaintext: &[u8], key: &AesKey) -> Vec<u8> {
        let mut output = Vec::with_capacity(plaintext.len() + 24);
        let iv = rand::thread_rng().gen::<[u8; 12]>();

        output.extend_from_slice(&iv);
        output.extend_from_slice(plaintext);

        let cipher = Aes128Gcm::new(key.as_ref().into());

        let tag = cipher
            .encrypt_in_place_detached(&iv.into(), &[], &mut output[12..])
            .unwrap();

        output.extend_from_slice(&tag[..12]);

        output
    }

    fn raw_decrypt(ciphertext: Vec<u8>, key: &AesKey) -> Result<String, NcrError> {
        if ciphertext.len() <= 24 {
            return Err(NcrError::DecryptError);
        }

        let iv: [u8; 12] = ciphertext[..12].try_into().unwrap();
        let tag: [u8; 12] = ciphertext[(ciphertext.len() - 12)..].try_into().unwrap();

        let mut output = Vec::from(&ciphertext[12..(ciphertext.len() - 12)]);

        let cipher = Aes128Gcm::new(key.as_ref().into());

        let _ = cipher.decrypt_in_place_detached(&iv.into(), &[], &mut output, &[0u8; 16].into());

        // TODO: Will be improved after aes-gcm crate is updated.
        let real_tag = cipher
            .encrypt_in_place_detached(&iv.into(), &[], &mut output.clone())
            .unwrap();

        if real_tag[..12].ct_eq(&tag).into() {
            String::from_utf8(output).map_err(|_| NcrError::DecryptError)
        } else {
            Err(NcrError::DecryptError)
        }
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
