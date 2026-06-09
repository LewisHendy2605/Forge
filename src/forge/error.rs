use std::io;
use thiserror::Error;

use super::counting_writer::CountingWriter;

#[derive(Error, Debug)]
pub enum ForgeError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("CSV error: {0}")]
    Csv(#[from] csv::Error),
    #[error("CSV flush error: {0}")]
    CsvIntoInner(#[from] csv::IntoInnerError<CountingWriter<std::fs::File>>),
    #[error("XML error: {0}")]
    Xml(#[from] xml::writer::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}
