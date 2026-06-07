use std::io;
use thiserror::Error;

use super::counting_writer::CountingWriter;

#[derive(Error, Debug)]
pub enum ForgeError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("CSV error: {0}")]
    Csv(#[from] csv::Error),
    #[error("XML error: {0}")]
    Xml(#[from] xml::writer::Error),
    #[error("CSV flush error: {0}")]
    CsvIntoInner(#[from] csv::IntoInnerError<CountingWriter<std::fs::File>>),
}
