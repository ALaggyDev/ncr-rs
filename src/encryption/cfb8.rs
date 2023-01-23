use aes::{
    cipher::{AsyncStreamCipher, KeyIvInit},
    Aes128,
};
use cfb8::{Decryptor, Encryptor};
use rand::Rng;
use std::{convert::Infallible, marker::PhantomData, num::Wrapping};

use super::Encryption;
use crate::{encoding::Encoding, AesKey, NcrError};

/// The aes/cfb8 encryption.
#[derive(Debug)]
pub struct Cfb8Encryption<E: Encoding>(PhantomData<E>);

impl<E: Encoding> Cfb8Encryption<E> {
    fn raw_encrypt(plaintext: &[u8], key: &AesKey) -> Vec<u8> {
        let mut output = Vec::with_capacity(8 + plaintext.len());
        let nonce = rand::thread_rng().gen::<[u8; 8]>();

        output.extend_from_slice(&nonce);
        output.extend_from_slice(plaintext);

        let iv = generate_iv(u64::from_be_bytes(nonce));

        Encryptor::<Aes128>::new(key.as_ref().into(), &iv.into()).encrypt(&mut output[8..]);

        output
    }

    fn raw_decrypt(ciphertext: Vec<u8>, key: &AesKey) -> Result<String, NcrError> {
        if ciphertext.len() < 8 {
            return Err(NcrError::DecryptError);
        }
        let nonce: [u8; 8] = ciphertext[..8].try_into().unwrap();

        let iv = generate_iv(u64::from_be_bytes(nonce));

        let mut output = Vec::from(&ciphertext[8..]);
        Decryptor::<Aes128>::new(key.as_ref().into(), &iv.into()).decrypt(&mut output);

        String::from_utf8(output).map_err(|_| NcrError::DecryptError)
    }
}

fn generate_iv(nonce: u64) -> [u8; 16] {
    /// Modulus
    const M: Wrapping<i64> = Wrapping((1 << 48) - 1);
    /// Multiplier
    const A: Wrapping<i64> = Wrapping(0x5DEECE66D);
    /// Increment
    const C: Wrapping<i64> = Wrapping(11);

    let mut iv = [0u8; 16];

    let mut state = Wrapping((nonce as i64) ^ A.0) & M;

    for chunk in iv.chunks_exact_mut(4) {
        state = (state * A + C) & M;

        chunk.copy_from_slice(&(((state.0 as u64) >> 16) as i32).to_le_bytes());
    }

    iv
}

impl<E: Encoding> Encryption for Cfb8Encryption<E> {
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
