#![allow(non_snake_case)]
#[macro_use]
extern crate rocket;
mod find;
mod functions;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

// {"08934":420696969}

#[post("/query-one/<table>", data = "<data>")]
async fn queryOne(table: &str, data: &str) -> String {
    println!("Table: {}", table);
    println!("Data: {}", data);
    let extension: &str = ".json";
    let together = format!("{}{}", table, extension);
    let query: &str = functions::query_format(data);
    let result = find::search_one(&together, query);
    let result_string = match result {
        Ok(matching_line) => format!("{:?}", matching_line),
        Err(error) => format!(r#"{{"error":"{}"}}"#, error),
    };
    result_string
}

#[post("/query-many/<table>", data = "<data>")]
async fn queryMany(table: &str, data: &str) -> String {
    println!("Table: {}", table);
    println!("Data: {}", data);
    let extension: &str = ".json";
    let together = format!("{}{}", table, extension);
    let query: &str = functions::query_format(data);
    let result = find::search_many(&together, query);
    let result_string = match result {
        Ok(matching_lines) => format!("{:?}", matching_lines),
        Err(error) => format!(r#"{{"error":"{}"}}"#, error),
    };
    result_string
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, queryOne, queryMany])
}