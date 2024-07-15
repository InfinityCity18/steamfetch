#![allow(dead_code)]

use applist_query::{find_best_app_id, get_applist};

mod appimage;
mod appinfo_json;
mod appinfo_query;
mod applist_json;
mod applist_query;
mod appprint;
mod arg;
mod error;
mod glyphs;

fn main() {
    /*
    println!("Hello, world!");
    let parser = arg::ArgParser::new();
    println!("{:#?}", parser);
    */
    let appid = find_best_app_id(
        get_applist("https://api.steampowered.com/ISteamApps/GetAppList/v1"),
        "grand theft auto",
    );
    let data = appinfo_query::get_app_info(appid).unwrap();
    //println!("{:#?}", data);

    appimage::print_image(&data.data.header_image, 48);
}
