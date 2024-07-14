#![allow(dead_code)]

mod appinfo_json;
mod appinfo_query;
mod applist_json;
mod applist_query;
mod arg;
mod error;

fn main() {
    println!("Hello, world!");
    let parser = arg::ArgParser::new();
    println!("{:#?}", parser);
}
