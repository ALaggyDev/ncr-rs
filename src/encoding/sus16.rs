use super::Encoding;
use crate::NcrError;

/// The sus16 encoding, made by [EnderKill98](https://github.com/EnderKill98).
///
/// See [No Chat Reports](https://github.com/HKS-HNS/No-Chat-Reports).
#[derive(Debug)]
pub struct Sus16Encoding;

impl Encoding for Sus16Encoding {
    fn encode(text: &[u8]) -> String {
        let mut output = String::new();

        for ch in text {
            output.push(SUS16_ENCODE[(*ch >> 4) as usize]);
            output.push(SUS16_ENCODE[(*ch & 0xf) as usize]);
        }

        output
    }

    fn decode(text: &str) -> Result<Vec<u8>, NcrError> {
        let mut output: Vec<u8> = Vec::new();

        let mut chars = text.chars();
        loop {
            let Some(c1) = chars.next() else { break; };
            let c2 = chars.next().ok_or(NcrError::DecodeError)?;

            let r1 = sus16_decode(c1)?;
            let r2 = sus16_decode(c2)?;

            output.push((r1 << 4) | r2);
        }

        Ok(output)
    }
}

const SUS16_ENCODE: [char; 16] = [
    'ඔ', 'ඕ', 'ඖ', 'ඞ', 'ච', 'ඩ', 'ඬ', 'ධ', 'ඹ', 'ව', 'ဨ', '၅', '၆', '၉', 'ၡ', 'ဥ',
];

#[rustfmt::skip]
fn sus16_decode(ch: char) -> Result<u8, NcrError> {
    Ok(match ch {
        'ඔ' => 0, 'ඕ' => 1, 'ඖ' => 2, 'ඞ' => 3,
        'ච' => 4, 'ඩ' => 5, 'ඬ' => 6, 'ධ' => 7,
        'ඹ' => 8, 'ව' => 9, 'ဨ' => 10, '၅' => 11,
        '၆' => 12, '၉' => 13, 'ၡ' => 14, 'ဥ' => 15,
        _ => return Err(NcrError::DecodeError),
    })
}
