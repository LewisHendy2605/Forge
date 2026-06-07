use std::fs::File;
use std::fs::OpenOptions;
use std::io;

// Opens file and returns file handle
pub fn open_file(path: &str) -> Result<File, io::Error> {
    OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(path)
}

pub fn format_bytes(bytes: u64) -> String {
    match bytes {
        b if b >= 1_073_741_824 => format!("{:.2} GB", b as f64 / 1_073_741_824.0),
        b if b >= 1_048_576 => format!("{:.2} MB", b as f64 / 1_048_576.0),
        b if b >= 1_024 => format!("{:.2} KB", b as f64 / 1_024.0),
        b => format!("{} B", b),
    }
}
