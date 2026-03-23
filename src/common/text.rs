use unicode_normalization::UnicodeNormalization;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DocumentType {
    Pdf,
    Xlsx,
    Docx,
    PlainText,
    Unknown,
}

fn contains_str(data: &[u8], pattern: &[u8]) -> bool {
    if pattern.is_empty() || data.len() < pattern.len() {
        return false;
    }
    for i in 0..=(data.len() - pattern.len()) {
        if data[i..].starts_with(pattern) {
            return true;
        }
    }
    false
}

pub fn detect_document_type(data: &[u8]) -> DocumentType {
    if data.len() < 4 {
        return DocumentType::Unknown;
    }

    if data.starts_with(b"%PDF") {
        return DocumentType::Pdf;
    }

    if data.starts_with(b"PK")
        && (data[2] == 0x03 || data[2] == 0x05 || data[2] == 0x07)
        && (data[3] == 0x04 || data[3] == 0x06 || data[3] == 0x08)
    {
        let check_len = data.len().min(8192);
        if contains_str(&data[..check_len], b"xl/worksheets/")
            || contains_str(&data[..check_len], b"xl/sharedStrings")
            || contains_str(&data[..check_len], b"xl/")
        {
            return DocumentType::Xlsx;
        }
        if contains_str(&data[..check_len], b"word/")
            || contains_str(&data[..check_len], b"word/document")
        {
            return DocumentType::Docx;
        }
    }

    if std::str::from_utf8(data).is_ok() {
        return DocumentType::PlainText;
    }

    DocumentType::Unknown
}

pub fn normalize_text(text: &str) -> String {
    text.lines()
        .map(|line| {
            line.chars()
                .filter(|c| {
                    c.is_alphanumeric()
                        || c.is_whitespace()
                        || matches!(
                            c,
                            '.' | ','
                                | ':'
                                | ';'
                                | '!'
                                | '?'
                                | '\''
                                | '"'
                                | '-'
                                | '('
                                | ')'
                                | '/'
                                | '@'
                                | '#'
                                | '&'
                                | '='
                                | '+'
                                | '*'
                                | '%'
                                | '•'
                                | '●'
                                | '©'
                                | '®'
                                | '™'
                                | '…'
                        )
                })
                .filter(|c| {
                    let code = *c as u32;
                    code < 0x80 || code > 0x9F
                })
                .nfc()
                .collect::<String>()
                .split_whitespace()
                .collect::<Vec<_>>()
                .join(" ")
        })
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}
