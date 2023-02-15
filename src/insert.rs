#![allow(non_snake_case)]
use std::io::prelude::*;
use std::fs::OpenOptions;

pub fn insert(filename: &str, insert_string: &str) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .open(filename)?;
    file.write_all(b"\n")?;
    file.write_all(insert_string.as_bytes())?;
    Ok(())
}