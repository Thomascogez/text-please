use crate::common::normalize_text;
use quick_xml::events::Event;
use quick_xml::Reader;
use std::io::Cursor;
use zip::ZipArchive;

fn extract_docx_text(docx_data: &[u8]) -> Result<String, String> {
    let cursor = Cursor::new(docx_data);
    let mut archive = ZipArchive::new(cursor).map_err(|e| e.to_string())?;

    let mut content = Vec::new();
    if let Ok(mut file) = archive.by_name("word/document.xml") {
        std::io::Read::read_to_end(&mut file, &mut content).map_err(|e| e.to_string())?;
    }

    if content.is_empty() {
        return Ok(String::new());
    }

    let mut reader = Reader::from_reader(&content[..]);
    reader.config_mut().trim_text(true);

    let mut paragraphs: Vec<String> = Vec::new();
    let mut current_text = String::new();
    let mut in_t = false;

    loop {
        match reader.read_event() {
            Ok(Event::Start(e)) => {
                let name_bytes = e.name().0.to_vec();
                let name = String::from_utf8_lossy(&name_bytes);
                if name.ends_with(":p") || name == "p" {
                    current_text.clear();
                } else if name.ends_with(":t") || name == "t" {
                    in_t = true;
                }
            }
            Ok(Event::End(e)) => {
                let name_bytes = e.name().0.to_vec();
                let name = String::from_utf8_lossy(&name_bytes);
                if name.ends_with(":p") || name == "p" {
                    let trimmed = current_text.trim();
                    if !trimmed.is_empty() {
                        paragraphs.push(trimmed.to_string());
                    }
                    current_text.clear();
                } else if name.ends_with(":t") || name == "t" {
                    in_t = false;
                }
            }
            Ok(Event::Text(e)) => {
                if in_t {
                    let text = String::from_utf8_lossy(&e.into_inner()).to_string();
                    if !current_text.is_empty() && !text.is_empty() {
                        current_text.push(' ');
                    }
                    current_text.push_str(&text);
                }
            }
            Ok(Event::Eof) | Err(_) => break,
            _ => {}
        }
    }

    let text = paragraphs.join("\n");
    Ok(normalize_text(&text))
}

pub fn extract_docx_to_text(docx: &[u8]) -> Result<String, String> {
    extract_docx_text(docx)
}
