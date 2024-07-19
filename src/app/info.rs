mod json;

pub use json::AppInfoRoot;

use serde_json::Value;

use crate::error::{ExitResult, IntoResultExitError};

use super::links::{APP_INFO, APP_PLAYERS};

impl AppInfoRoot {
    pub fn get_app_info(app_id: u32, lang: &str) -> ExitResult<AppInfoRoot> {
        let url = APP_INFO
            .replace("{1}", &app_id.to_string())
            .replace("{2}", lang);

        let response = reqwest::blocking::get(url).into_exit_error("fetching app info failed")?;

        let mut data: Value = response.json().into_exit_error("parsing json failed")?;

        serde_json::from_value(data[app_id.to_string()].take())
            .into_exit_error("parsing json failed")
    }

    pub fn get_player_count(app_id: u32) -> ExitResult<'static, u32> {
        let url = APP_PLAYERS.replace("{}", &app_id.to_string());

        let response =
            reqwest::blocking::get(url).into_exit_error("fetching player count failed")?;

        let mut data: Value = response.json().into_exit_error("parsing json failed")?;

        serde_json::from_value(data["response"]["player_count"].take())
            .into_exit_error("parsing json failed")
    }
}
