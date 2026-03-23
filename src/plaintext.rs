use crate::common::normalize_text;

pub fn extract_plaintext_to_text(data: &[u8]) -> Result<String, String> {
    match std::str::from_utf8(data) {
        Ok(text) => Ok(normalize_text(text)),
        Err(e) => Err(format!("Invalid UTF-8: {}", e)),
    }
}
