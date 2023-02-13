use std::collections::VecDeque;
use std::fs::File;
use std::io::BufWriter;
use std::io::{BufRead, BufReader, Write};

pub fn search_one(filename: &str, search_string: &str) -> Result<String, std::io::Error> {
    let file = File::open(filename)?;
    let mut reader = BufReader::with_capacity(2048 * 2048, file);
		let start = std::time::Instant::now();
    loop {
        let mut line = String::new();
        let n = reader.read_line(&mut line)?;
        if n == 0 {
            break;
        }
        if line.contains(search_string) {
						let elapsed = start.elapsed();
            println!("Found query in {:?}", elapsed);
            return Ok(line);
        }
    }
    Ok(String::new())
}

pub fn search_many(filename: &str, search_string: &str) -> Result<Vec<String>, std::io::Error> {
    let file = File::open(filename)?;
    let mut reader = BufReader::with_capacity(2048 * 2048, file);
		let start = std::time::Instant::now();
    let mut matching_lines = vec![];
    loop {
        let mut line = String::new();
        let n = reader.read_line(&mut line)?;
        if n == 0 {
            break;
        }
        if line.contains(search_string) {
            matching_lines.push(line);
        }
    }
    let elapsed = start.elapsed();
    println!("Found query in {:?}", elapsed);
    Ok(matching_lines)
}