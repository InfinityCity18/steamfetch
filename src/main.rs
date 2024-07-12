#![allow(dead_code)]

mod arg;
mod dns;
mod error;
mod net;

fn main() {
    println!("Hello, world!");
    let parser = arg::ArgParser::new();
    println!("{:#?}", parser);
    let domain = dns::dns_query("store.steampowered.com");
    println!("{:#?}", domain);
}
