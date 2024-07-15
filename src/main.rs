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
        "grand theft auto iv",
    );
    let data = appinfo_query::get_app_info(appid).unwrap();
    //println!("{:#?}", data);
    let width = 48;
    let height = (width * 10) / 46;
    let offset = 0;

    appprint::print_app_info::<crate::glyphs::FancyFont>(data, width, height, offset);
}
