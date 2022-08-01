use std::io::{Read, Write};
use std::fs::File;
use std::fs::OpenOptions;

pub fn open_read(path: &str) -> impl Read {

    File::open(path).unwrap()
}

pub fn open_write(path: &str) -> impl Write {
    OpenOptions::new()
    .write(true)
    .append(true)
    .open(path)
    .unwrap()
}
