use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppInfoRoot {
    pub success: bool,
    pub data: AppInfo,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppInfo {
    pub name: String,
    #[serde(rename = "steam_appid")]
    pub steam_appid: u32,
    #[serde(rename = "is_free")]
    pub is_free: bool,
    #[serde(rename = "short_description")]
    pub short_description: String,
    #[serde(rename = "header_image")]
    pub header_image: String,
    pub developers: Vec<String>,
    pub publishers: Vec<String>,
    #[serde(rename = "price_overview")]
    pub price_overview: Option<PriceOverview>,
    pub platforms: Platforms,
    #[serde(rename = "release_date")]
    pub release_date: ReleaseDate,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Platforms {
    pub windows: bool,
    pub mac: bool,
    pub linux: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReleaseDate {
    #[serde(rename = "coming_soon")]
    pub coming_soon: bool,
    pub date: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceOverview {
    pub currency: String,
    pub initial: i64,
    #[serde(rename = "final")]
    pub final_field: i64,
    #[serde(rename = "discount_percent")]
    pub discount_percent: i64,
    #[serde(rename = "initial_formatted")]
    pub initial_formatted: String,
    #[serde(rename = "final_formatted")]
    pub final_formatted: String,
}
