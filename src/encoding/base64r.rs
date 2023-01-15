use base64::{engine::general_purpose::STANDARD, Engine};

use super::Encoding;
use crate::NcrError;

/// The base64r encoding, made by [No Chat Reports](https://github.com/HKS-HNS/No-Chat-Reports).
///
/// In Minecraft 1.19.3, the character `¸` is changed to `×`.
/// If you are using 1.19.3 or above, please use [NewBase64rEncoding] instead.
#[derive(Debug)]
pub struct Base64rEncoding;

impl Encoding for Base64rEncoding {
    fn encode(text: &[u8]) -> String {
        let encoded = STANDARD.encode(text);
        let mut output = String::new();

        for ch in encoded.chars() {
            output.push(unsafe { *BASE64R_ENCODE.get(&ch).unwrap_unchecked() });
        }

        output
    }

    fn decode(text: &str) -> Result<Vec<u8>, NcrError> {
        let mut output = String::new();

        for ch in text.chars() {
            output.push(*BASE64R_DECODE.get(&ch).ok_or(NcrError::DecodeError)?);
        }

        STANDARD.decode(output).map_err(|_| NcrError::DecodeError)
    }
}

/// The new base64r encoding, made by [No Chat Reports](https://github.com/HKS-HNS/No-Chat-Reports).
///
/// In Minecraft 1.19.3, the character `¸` is changed to `×`.
/// If you are using 1.19.2, please use [Base64rEncoding] instead.
#[derive(Debug)]
pub struct NewBase64rEncoding;

impl Encoding for NewBase64rEncoding {
    fn encode(text: &[u8]) -> String {
        let encoded = STANDARD.encode(text);
        let mut output = String::new();

        for ch in encoded.chars() {
            let new_ch = if ch == 'x' {
                '×'
            } else {
                unsafe { *BASE64R_ENCODE.get(&ch).unwrap_unchecked() }
            };

            output.push(new_ch);
        }

        output
    }

    fn decode(text: &str) -> Result<Vec<u8>, NcrError> {
        let mut output = String::new();

        for ch in text.chars() {
            let new_ch = if ch == '×' {
                'x'
            } else {
                *BASE64R_DECODE.get(&ch).ok_or(NcrError::DecodeError)?
            };

            output.push(new_ch);
        }

        STANDARD.decode(output).map_err(|_| NcrError::DecodeError)
    }
}

// Post-expanded macros:

#[rustfmt::skip]
const BASE64R_ENCODE: phf::Map<char, char> = phf::Map {
    key: 15467950696543387533u64,
    disps: &[
        (6u32, 14u32), (3u32, 2u32), (0u32, 0u32), (6u32, 6u32),
        (3u32, 30u32), (1u32, 47u32), (0u32, 3u32), (17u32, 11u32),
        (4u32, 64u32), (0u32, 1u32), (0u32, 0u32), (0u32, 13u32),
        (0u32, 8u32),
    ],
    entries: &[
        ('V', ']'), ('Q', '>'), ('K', '-'), ('p', '¯'),
        ('t', '³'), ('8', '8'), ('r', '±'), ('u', 'µ'),
        ('5', '5'), ('d', '¡'), ('i', '¦'), ('c', '~'),
        ('6', '6'), ('S', '@'), ('=', '¿'), ('3', '3'),
        ('a', '|'), ('U', '\\'), ('R', '?'), ('+', '+'),
        ('D', '$'), ('q', '°'), ('W', '^'), ('9', '9'),
        ('e', '¢'), ('E', '%'), ('N', ';'), ('Z', '{'),
        ('P', '='), ('T', '['), ('v', '¶'), ('A', '!'),
        ('H', '('), ('z', 'º'), ('s', '²'), ('k', '©'),
        ('O', '<'), ('h', '¥'), ('f', '£'), ('F', '¼'),
        ('o', '®'), ('4', '4'), ('M', ':'), ('0', '0'),
        ('2', '2'), ('1', '1'), ('j', '¨'), ('l', 'ª'),
        ('L', '.'), ('Y', '`'), ('J', ','), ('w', '·'),
        ('b', '}'), ('g', '¤'), ('7', '7'), ('B', '"'),
        ('G', '\''), ('x', '¸'), ('y', '¹'), ('/', '»'),
        ('X', '_'), ('I', ')'), ('m', '«'), ('C', '#'),
        ('n', '¬'),
    ],
};

#[rustfmt::skip]
const BASE64R_DECODE: phf::Map<char, char> = phf::Map {
    key: 15467950696543387533u64,
    disps: &[
        (11u32, 49u32), (0u32, 64u32), (0u32, 0u32), (0u32, 20u32),
        (7u32, 44u32), (6u32, 54u32), (10u32, 25u32), (0u32, 1u32),
        (41u32, 56u32), (1u32, 0u32), (4u32, 35u32), (1u32, 0u32),
        (45u32, 42u32),
    ],
    entries: &[
        ('·', 'w'), ('¤', 'g'), ('\'', 'G'), ('¸', 'x'),
        ('«', 'm'), ('^', 'W'), ('¥', 'h'), ('`', 'Y'),
        ('2', '2'), ('¢', 'e'), ('\\', 'U'), ('7', '7'),
        ('(', 'H'), (';', 'N'), ('=', 'P'), ('£', 'f'),
        ('°', 'q'), ('µ', 'u'), ('<', 'O'), ('¡', 'd'),
        ('¨', 'j'), ('3', '3'), ('¶', 'v'), ('~', 'c'),
        ('¬', 'n'), ('@', 'S'), ('©', 'k'), ('¯', 'p'),
        ('9', '9'), ('?', 'R'), ('{', 'Z'), ('|', 'a'),
        ('ª', 'l'), ('-', 'K'), ('_', 'X'), ('¦', 'i'),
        ('$', 'D'), ('4', '4'), ('³', 't'), ('[', 'T'),
        ('8', '8'), ('#', 'C'), ('"', 'B'), ('²', 's'),
        ('+', '+'), (',', 'J'), ('.', 'L'), (')', 'I'),
        ('®', 'o'), ('º', 'z'), ('6', '6'), (']', 'V'),
        ('»', '/'), ('1', '1'), ('0', '0'), ('±', 'r'),
        ('!', 'A'), ('>', 'Q'), ('¼', 'F'), ('¿', '='),
        (':', 'M'), ('5', '5'), ('%', 'E'), ('}', 'b'),
        ('¹', 'y'),
    ],
};

// Pre-expanded macros:
// phf = { version = "0.11.1", features = ["macros"] }

// const BASE64R_ENCODE: phf::Map<char, char> = phf::phf_map! {
//     'A' => '!', 'B' => '"', 'C' => '#', 'D' => '$',
//     'E' => '%', 'F' => '¼', 'G' => '\'', 'H' => '(',
//     'I' => ')', 'J' => ',', 'K' => '-', 'L' => '.',
//     'M' => ':', 'N' => ';', 'O' => '<', 'P' => '=',
//     'Q' => '>', 'R' => '?', 'S' => '@', 'T' => '[',
//     'U' => '\\', 'V' => ']', 'W' => '^', 'X' => '_',
//     'Y' => '`', 'Z' => '{', 'a' => '|', 'b' => '}',
//     'c' => '~', 'd' => '¡', 'e' => '¢', 'f' => '£',
//     'g' => '¤', 'h' => '¥', 'i' => '¦', 'j' => '¨',
//     'k' => '©', 'l' => 'ª', 'm' => '«', 'n' => '¬',
//     'o' => '®', 'p' => '¯', 'q' => '°', 'r' => '±',
//     's' => '²', 't' => '³', 'u' => 'µ', 'v' => '¶',
//     'w' => '·', 'x' => '¸', 'y' => '¹', 'z' => 'º',
//     '0' => '0', '1' => '1', '2' => '2', '3' => '3',
//     '4' => '4', '5' => '5', '6' => '6', '7' => '7',
//     '8' => '8', '9' => '9', '+' => '+', '/' => '»',
//     '=' => '¿',
// };
//
// const BASE64R_DECODE: phf::Map<char, char> = phf::phf_map! {
//     '!' => 'A', '"' => 'B', '#' => 'C', '$' => 'D',
//     '%' => 'E', '¼' => 'F', '\'' => 'G', '(' => 'H',
//     ')' => 'I', ',' => 'J', '-' => 'K', '.' => 'L',
//     ':' => 'M', ';' => 'N', '<' => 'O', '=' => 'P',
//     '>' => 'Q', '?' => 'R', '@' => 'S', '[' => 'T',
//     '\\' => 'U', ']' => 'V', '^' => 'W', '_' => 'X',
//     '`' => 'Y', '{' => 'Z', '|' => 'a', '}' => 'b',
//     '~' => 'c', '¡' => 'd', '¢' => 'e', '£' => 'f',
//     '¤' => 'g', '¥' => 'h', '¦' => 'i', '¨' => 'j',
//     '©' => 'k', 'ª' => 'l', '«' => 'm', '¬' => 'n',
//     '®' => 'o', '¯' => 'p', '°' => 'q', '±' => 'r',
//     '²' => 's', '³' => 't', 'µ' => 'u', '¶' => 'v',
//     '·' => 'w', '¸' => 'x', '¹' => 'y', 'º' => 'z',
//     '0' => '0', '1' => '1', '2' => '2', '3' => '3',
//     '4' => '4', '5' => '5', '6' => '6', '7' => '7',
//     '8' => '8', '9' => '9', '+' => '+', '»' => '/',
//     '¿' => '=',
// };
