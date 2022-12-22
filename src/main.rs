#[macro_use] extern crate rocket;
use std::collections::VecDeque;
use std::fs::File;
use std::io::BufWriter;
use std::io::{BufRead, BufReader, Write};

fn search_one(filename: &str, search_string: &str) -> Result<String, std::io::Error> {
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
            println!("Line containing {} found: {}", search_string, line);
            return Ok(line);
        }
    }
    let elapsed = start.elapsed();
    println!("Elapsed time: {:?}", elapsed);
    Ok(String::new())
}

#[get("/")]
fn index() -> String {
    let result = search_one("Test.json", r#""08934":420696969"#);
    let result_string = match result {
        Ok(matching_line) => format!("{}", matching_line),
        Err(error) => format!("An error occurred: {}", error),
    };
    result_string
}



#[get("/test")]
fn test() -> String {
    format!("Hello, test")
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![test])
}
