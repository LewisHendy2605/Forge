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
