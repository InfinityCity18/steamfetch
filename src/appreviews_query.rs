use serde_json::Value;

use crate::{appreviews_json::QuerySummary, error};

pub fn get_app_reviews(appid: u32) -> QuerySummary {
    let url = format!("https://store.steampowered.com/appreviews/{}?json=1?filter=all?review_type=all&purchase_type=all&language=all&num_per_page=0", appid);
    let response = reqwest::blocking::get(url)
        .unwrap_or_else(|_| error::error_and_quit("failed to get app reviews"));

    let mut reviews: Value = response
        .json()
        .unwrap_or_else(|_| error::error_and_quit("failed to parse reviews json"));

    serde_json::from_value(reviews["query_summary"].take()).expect("parsing failed")
}
