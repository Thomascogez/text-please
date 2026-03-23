use crate::common::normalize_text;
use quick_xml::events::Event;
use quick_xml::Reader;
use std::io::Cursor;
use zip::ZipArchive;

fn read_shared_strings(archive: &mut ZipArchive<Cursor<&[u8]>>) -> Vec<String> {
    let mut strings = Vec::new();

    if let Ok(mut file) = archive.by_name("xl/sharedStrings.xml") {
        let mut content = Vec::new();
        if std::io::Read::read_to_end(&mut file, &mut content).is_err() {
            return strings;
        }

        let mut reader = Reader::from_reader(&content[..]);
        reader.config_mut().trim_text(true);

        let mut current_string = String::new();

        loop {
            match reader.read_event() {
                Ok(Event::Start(ref e)) if e.name().as_ref() == b"t" => {
                    current_string.clear();
                }
                Ok(Event::End(ref e)) if e.name().as_ref() == b"si" => {
                    strings.push(current_string.clone());
                }
                Ok(Event::Text(e)) => {
                    let text = String::from_utf8_lossy(&e.into_inner()).to_string();
                    current_string.push_str(&text);
                }
                Ok(Event::Eof) | Err(_) => break,
                _ => {}
            }
        }
    }

    strings
}

fn extract_xlsx_data(xlsx: &[u8]) -> Result<String, String> {
    let cursor = Cursor::new(xlsx);
    let mut archive = ZipArchive::new(cursor).map_err(|e| e.to_string())?;

    let shared_strings = read_shared_strings(&mut archive);

    let sheet_files: Vec<String> = archive
        .file_names()
        .filter(|name| name.starts_with("xl/worksheets/sheet") && name.ends_with(".xml"))
        .map(|s| s.to_string())
        .collect();

    let mut all_rows: Vec<String> = Vec::new();

    for sheet_file in sheet_files {
        if let Ok(mut file) = archive.by_name(&sheet_file) {
            let mut content = Vec::new();
            if std::io::Read::read_to_end(&mut file, &mut content).is_err() {
                continue;
            }

            let mut reader = Reader::from_reader(&content[..]);
            reader.config_mut().trim_text(true);

            let mut current_row: Vec<String> = Vec::new();
            let mut current_cell = String::new();
            let mut cell_type: Option<String> = None;

            loop {
                match reader.read_event() {
                    Ok(Event::Start(ref e)) => {
                        let name = e.name();
                        if name.as_ref() == b"c" {
                            current_cell.clear();
                            cell_type = None;

                            for attr in e.attributes().flatten() {
                                let key = attr.key.as_ref();
                                let value = String::from_utf8_lossy(&attr.value).to_string();
                                if key == b"t" {
                                    cell_type = Some(value);
                                }
                            }
                        }
                    }
                    Ok(Event::End(ref e)) => {
                        let name = e.name();
                        if name.as_ref() == b"c" {
                            current_row.push(current_cell.clone());
                            current_cell.clear();
                        } else if name.as_ref() == b"row" {
                            if !current_row.is_empty() {
                                all_rows.push(current_row.join("\t"));
                            }
                            current_row.clear();
                        }
                    }
                    Ok(Event::Text(e)) => {
                        let text = String::from_utf8_lossy(&e.into_inner()).to_string();
                        if !text.is_empty() {
                            if cell_type.as_deref() == Some("s") {
                                if let Ok(index) = text.parse::<usize>() {
                                    if index < shared_strings.len() {
                                        current_cell = shared_strings[index].clone();
                                    }
                                }
                            } else {
                                current_cell = text;
                            }
                        }
                    }
                    Ok(Event::Eof) | Err(_) => break,
                    _ => {}
                }
            }
        }
    }

    Ok(normalize_text(&all_rows.join("\n")))
}

pub fn extract_xlsx_to_text(xlsx: &[u8]) -> Result<String, String> {
    extract_xlsx_data(xlsx)
}
