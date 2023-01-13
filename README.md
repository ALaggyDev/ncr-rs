# No Chat Reports (NCR) Chat Encryption

This crate implements the [No Chat Reports](https://github.com/Aizistral-Studios/No-Chat-Reports)'s custom chat encryption.
More specifically this implements a fork of [No Chat Reports](https://github.com/HKS-HNS/No-Chat-Reports).

All functionalities of the custom chat encryption are implemented.
You can still use this crate normally if you are using the original [No Chat Reports](https://github.com/Aizistral-Studios/No-Chat-Reports).

- Caesar, Ecb, Cfb8 and Gcm encryption
- Base64 (old), Base64r, [Sus16](https://github.com/HKS-HNS/No-Chat-Reports) and [Mc256](https://github.com/HKS-HNS/No-Chat-Reports) encoding
- Passphrase

# Examples

## Encrypting

```rust
use ncr::{
    encoding::Base64rEncoding,
    encryption::{Cfb8Encryption, Encryption},
    utils::prepend_header,
    AesKey,
};

let key = AesKey::gen_from_passphrase(b"secret");

let plaintext = prepend_header("I love Minecraft!");
let ciphertext = Cfb8Encryption::<Base64rEncoding>::encrypt(&plaintext, &key).unwrap();

println!("{}", ciphertext);
```

## Decrypting

```rust
use ncr::{
    encoding::Base64rEncoding,
    encryption::{Cfb8Encryption, Encryption},
    utils::trim_header,
    AesKey,
};

let key = AesKey::gen_from_passphrase(b"secret");

let ciphertext = r#"%[2_0»³"!7).«?;!.$¥`¶:8~667ª¸[¬)¢+¤^"#;
let plaintext = Cfb8Encryption::<Base64rEncoding>::decrypt(ciphertext, &key).unwrap();

let plaintext = trim_header(&plaintext).unwrap();

assert_eq!(plaintext, "I love Minecraft!");
```