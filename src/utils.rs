//! Some common utility functions.

use crate::NcrError;

/// Append "#%" before a text.
#[inline]
pub fn prepend_header(text: &str) -> String {
    "#%".to_owned() + text
}

/// Remove "#%" before a text.
///
/// # Error
///
/// This return a error if "#%" is not presented before the text.
#[inline]
pub fn trim_header(text: &str) -> Result<&str, NcrError> {
    if text.starts_with("#%") {
        Ok(text.split_at(2).1)
    } else {
        Err(NcrError::HeaderError)
    }
}
