use xml::writer::{EmitterConfig, XmlEvent};

use super::error::ForgeError;
use super::generator::generate_data;
use super::utils::open_file;

pub fn write(path: &str, records: u32, columns: Vec<String>) -> Result<(), ForgeError> {
    let file = open_file(&path)?;
    let mut writer = EmitterConfig::new()
        .perform_indent(true)
        .create_writer(file);

    let mut headers: Vec<(String, String)> = Vec::new();

    // Parse schema
    for col in columns {
        let (col_name, col_type) = col.split_once(":").unwrap_or((&col, "string"));

        headers.push((col_name.trim().to_string(), col_type.to_string()));
    }

    // Root element
    writer.write(XmlEvent::start_element("records"))?;

    // Data
    for i in 1..=records {
        writer.write(XmlEvent::start_element("record"))?;
        for (header_name, header_type) in &headers {
            let data = generate_data(&header_type, Some(i), None);
            writer.write(XmlEvent::start_element(header_name.as_str()))?;
            writer.write(XmlEvent::characters(&data))?;
            writer.write(XmlEvent::end_element())?;
        }
        writer.write(XmlEvent::end_element())?;
    }

    writer.write(XmlEvent::end_element())?;

    Ok(())
}
