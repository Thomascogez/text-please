pub mod common;
pub mod docx;
pub mod pdf;
pub mod plaintext;
pub mod xlsx;

pub use common::{DocumentType, detect_document_type};
pub use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = extractText)]
pub fn extract_text(data: &[u8]) -> Result<String, JsError> {
    match detect_document_type(data) {
        DocumentType::Pdf => pdf::extract_pdf_to_text(data).map_err(|e| JsError::new(&e)),
        DocumentType::Xlsx => xlsx::extract_xlsx_to_text(data).map_err(|e| JsError::new(&e)),
        DocumentType::Docx => docx::extract_docx_to_text(data).map_err(|e| JsError::new(&e)),
        DocumentType::PlainText => {
            plaintext::extract_plaintext_to_text(data).map_err(|e| JsError::new(&e))
        }
        DocumentType::Unknown => Err(JsError::new("Unknown document format")),
    }
}

#[wasm_bindgen(js_name = extractPdfToText)]
pub fn extract_pdf_to_text(pdf: &[u8]) -> Result<String, JsError> {
    pdf::extract_pdf_to_text(pdf).map_err(|e| JsError::new(&e))
}

#[wasm_bindgen(js_name = extractXlsxToText)]
pub fn extract_xlsx_to_text(xlsx: &[u8]) -> Result<String, JsError> {
    xlsx::extract_xlsx_to_text(xlsx).map_err(|e| JsError::new(&e))
}

#[wasm_bindgen(js_name = extractDocxToText)]
pub fn extract_docx_to_text(docx: &[u8]) -> Result<String, JsError> {
    docx::extract_docx_to_text(docx).map_err(|e| JsError::new(&e))
}

#[wasm_bindgen(js_name = extractPlaintextToText)]
pub fn extract_plaintext_to_text(data: &[u8]) -> Result<String, JsError> {
    plaintext::extract_plaintext_to_text(data).map_err(|e| JsError::new(&e))
}
