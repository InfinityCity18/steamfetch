#![allow(dead_code)]

mod arg;
mod error;

fn main() {
    println!("Hello, world!");
    let parser = arg::ArgParser::new();
}
