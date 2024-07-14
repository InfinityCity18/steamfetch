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
    #[serde(rename = "type")]
    pub type_field: String,
    pub name: String,
    #[serde(rename = "steam_appid")]
    pub steam_appid: i64,
    #[serde(rename = "is_free")]
    pub is_free: bool,
    #[serde(rename = "short_description")]
    pub short_description: String,
    #[serde(rename = "header_image")]
    pub header_image: String,
    pub developers: Vec<String>,
    pub publishers: Vec<String>,
    pub platforms: Platforms,
    pub recommendations: Recommendations,
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
pub struct Recommendations {
    pub total: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReleaseDate {
    #[serde(rename = "coming_soon")]
    pub coming_soon: bool,
    pub date: String,
}
