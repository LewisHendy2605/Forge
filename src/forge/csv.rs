use std::io;

use super::generator::generate_data;
use super::utils::open_file;

// Generates and writes csv file from input params
pub fn write(path: &str, lines: u32, delim: u8, columns: Vec<String>) -> Result<(), io::Error> {
    let file = open_file(&path)?;
    let mut writer = csv::WriterBuilder::new().delimiter(delim).from_writer(file);

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
    for i in 1..=lines {
        // Build row
        let mut row: Vec<String> = Vec::new();
        for t in &types {
            row.push(generate_data(t, Some(i), Some(delim)))
        }

        writer.write_record(row)?;
    }

    Ok(())
}
