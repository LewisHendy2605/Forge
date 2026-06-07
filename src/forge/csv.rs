use std::io;

use super::counting_writer::CountingWriter;
use super::error::ForgeError;
use super::generator::generate_data;
use super::utils;

// Generates and writes csv file from input params
pub fn write(path: &str, records: u32, delim: u8, columns: Vec<String>) -> Result<(), ForgeError> {
    let file = utils::open_file(&path)?;
    let counting = CountingWriter {
        inner: file,
        bytes: 0,
    };
    let mut writer = csv::WriterBuilder::new()
        .delimiter(delim)
        .from_writer(counting);

    let mut headers: Vec<String> = Vec::new();
    let mut types: Vec<String> = Vec::new();

    // Parse schema
    for col in columns {
        let (col_name, col_type) = col.split_once(":").unwrap_or((&col, "string"));

        headers.push(col_name.to_string());
        types.push(col_type.to_string());
    }

    writer.write_record(headers)?;

    // Data
    for i in 1..=records {
        // Build row
        let mut row: Vec<String> = Vec::new();
        for t in &types {
            row.push(generate_data(t, Some(i), Some(delim)))
        }

        writer.write_record(row)?;
    }

    writer.flush()?;

    let inner = writer
        .into_inner()
        .map_err(|e| ForgeError::Io(io::Error::new(io::ErrorKind::Other, e.to_string())))?;
    let bytes = inner.bytes;

    println!("Written: {}", utils::format_bytes(bytes));
    println!("File: {}", path);

    Ok(())
}
