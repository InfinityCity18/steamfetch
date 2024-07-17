#![allow(dead_code)]

use applist_query::{find_best_app_id, get_applist};

mod appimage;
mod appinfo_json;
mod appinfo_query;
mod applist_json;
mod applist_query;
mod appprint;
mod appreviews_json;
mod appreviews_query;
mod arg;
mod error;
mod glyphs;
mod print;

fn main() {
    /*
    println!("Hello, world!");
    let parser = arg::ArgParser::new();
    println!("{:#?}", parser);
    */
    let appid = find_best_app_id(
        get_applist("https://api.steampowered.com/ISteamApps/GetAppList/v1"),
        "english",
        "atomic heart",
    );
    let data = appinfo_query::get_app_info(appid, "english").unwrap();
    //println!("{:#?}", data);
    let width = 48;
    let height = (width * 10) / 46;
    let offset = 0;

    let test = crate::print::Character::create_vec_from_str("wda", "w", "a");

    appprint::print_app_info::<crate::glyphs::FancyFont>(data, width, height, offset);
}
