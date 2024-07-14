#![allow(dead_code)]

use viuer::{print_from_file, Config};

mod appimage;
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
    let data = appinfo_query::get_app_info(440);
    println!("{:#?}", data);

    let conf = Config {
        x: 1,
        y: 1,
        absolute_offset: false,
        ..Default::default()
    };
    // will resize the image to fit in 40x30 terminal cells and print it
    print_from_file("/home/hyperbarq/Downloads/ff13.jpg", &conf).expect("Image printing failed.");
}
