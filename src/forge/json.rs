use super::error::ForgeError;
use super::generator::generate_data;
use super::utils;
use serde_json::{Map, Value};
use std::io::BufWriter;

pub fn write(path: &str, records: u32, columns: Vec<String>) -> Result<(), ForgeError> {
    let file = utils::open_file(&path)?;
    let writer = BufWriter::new(file);
    let mut headers: Vec<(String, String)> = Vec::new();

    // Parse schema
    for col in columns {
        let (col_name, col_type) = col.split_once(":").unwrap_or((&col, "string"));

        headers.push((col_name.trim().to_string(), col_type.to_string()));
    }

    // Data
    let mut rows: Vec<Value> = Vec::new();
    for i in 1..=records {
        let mut map = Map::new();
        for (name, col_type) in &headers {
            let data = generate_data(col_type, Some(i), None);
            map.insert(name.clone(), Value::String(data));
        }
        rows.push(Value::Object(map));
    }

    serde_json::to_writer_pretty(writer, &rows)?;

    let bytes = std::fs::metadata(path)?.len();

    println!("Written: {}", utils::format_bytes(bytes));
    println!("File: {}", path);

    Ok(())
}
