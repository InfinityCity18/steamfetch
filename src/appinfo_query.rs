use crate::appinfo_json::AppInfoRoot;
use crate::error::error_and_quit;
use serde_json::Value;

pub fn get_app_info(appid: u32, lang: &str) -> Option<AppInfoRoot> {
    let url = format!(
        "https://store.steampowered.com/api/appdetails/?appids={}&l={}",
        appid, lang
    );

    println!("{}", url);

    let response =
        reqwest::blocking::get(url).unwrap_or_else(|_| error_and_quit("app info query failed"));

    let mut data: Value = response
        .json()
        .unwrap_or_else(|_| error_and_quit("app info json parsing failed"));

    println!("{:#?}", data[appid.to_string()]);

    serde_json::from_value(data[appid.to_string()].take()).ok()
}
