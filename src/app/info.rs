mod json;

pub use json::AppInfoRoot;

use serde_json::Value;

use crate::error::{ExitResult, IntoResultExitError};

use super::links::{APP_INFO, APP_PLAYERS};

impl AppInfoRoot {
    pub async fn get_app_info(app_id: u32, lang: &str) -> ExitResult<'static, AppInfoRoot> {
        let url = APP_INFO
            .replace("{1}", &app_id.to_string())
            .replace("{2}", lang);

        let response = reqwest::get(url)
            .await
            .into_exit_error("fetching app info failed")?;

        let mut data: Value = response
            .json()
            .await
            .into_exit_error("parsing json failed")?;

        serde_json::from_value(data[app_id.to_string()].take())
            .into_exit_error("parsing json failed")
    }

    pub async fn get_player_count(app_id: u32) -> ExitResult<'static, Option<u32>> {
        let url = APP_PLAYERS.replace("{}", &app_id.to_string());

        let response = reqwest::get(url)
            .await
            .into_exit_error("fetching player count failed")?;

        let mut data: Value = response
            .json()
            .await
            .into_exit_error("parsing json failed")?;

        Ok(serde_json::from_value(data["response"]["player_count"].take()).ok())
    }
}
