use crate::common::normalize_text;
use pdf_oxide::PdfDocument;

fn extract_pdf_text(pdf: &[u8]) -> Result<String, String> {
    let mut doc = PdfDocument::from_bytes(pdf.to_vec()).map_err(|e| e.to_string())?;

    let mut all_text = String::new();
    let page_count = doc.page_count().unwrap_or(0);

    for page_idx in 0..page_count {
        match doc.extract_text(page_idx) {
            Ok(text) => {
                if page_idx > 0 && !text.trim().is_empty() {
                    all_text.push('\n');
                }
                all_text.push_str(&text);
            }
            Err(_) => continue,
        }
    }

    Ok(normalize_text(&all_text))
}

pub fn extract_pdf_to_text(pdf: &[u8]) -> Result<String, String> {
    extract_pdf_text(pdf)
}
