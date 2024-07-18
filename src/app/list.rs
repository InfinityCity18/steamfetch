use json::{App, AppsList};

use crate::error::{ExitError, ExitResult, IntoResultExitError};
use serde_json::Value;
use std::cmp::Ordering;

mod json;

impl AppsList {
    pub fn get_applist(url: &str) -> ExitResult<AppsList> {
        let response = reqwest::blocking::get(url).into_exit_error("fetching applist failed")?;

        let mut list: Value = response.json().into_exit_error("parsing json failed")?;
        serde_json::from_value(list["applist"]["apps"].take())
            .into_exit_error("parsing json failed")
    }

    pub fn get_most_matching_app_id(&self, searched_app_name: &str, lang: &str) -> ExitResult<u32> {
        let mut best_ref: Option<&App> = None;
        let mut best_match_value: usize = usize::MAX;
        let mut zero_edit_shortest: usize = usize::MAX;

        for app in &self.apps {
            if app.name.to_lowercase() == searched_app_name.to_lowercase() {
                match super::info::AppInfoRoot::get_app_info(app.appid, lang) {
                    Ok(_) => return Ok(app.appid),
                    Err(_) => continue,
                }
            }
            let edit = get_edit_distance(&app.name, searched_app_name);

            if edit == 0 && app.name.len() <= zero_edit_shortest {
                match super::info::AppInfoRoot::get_app_info(app.appid, lang) {
                    Ok(_) => (),
                    Err(_) => continue,
                }
                zero_edit_shortest = app.name.len();
                best_ref = Some(app);
                best_match_value = edit;
            } else if edit < best_match_value {
                match super::info::AppInfoRoot::get_app_info(app.appid, lang) {
                    Ok(_) => (),
                    Err(_) => continue,
                }
                best_ref = Some(app);
                best_match_value = edit;
            }
        }

        if let Some(app) = best_ref {
            return Ok(app.appid);
        } else {
            return Err(ExitError("no matching app found"));
        }
    }
}

fn get_edit_distance(app_name: &str, searched_app_name: &str) -> usize {
    use edit_distance::edit_distance;

    match app_name
        .len()
        .partial_cmp(&searched_app_name.len())
        .expect("lengths should be comparable")
    {
        Ordering::Less => edit_distance(
            app_name.to_lowercase().as_ref(),
            searched_app_name.to_lowercase().as_ref(),
        ),
        Ordering::Equal => edit_distance(
            app_name.to_lowercase().as_ref(),
            searched_app_name.to_lowercase().as_ref(),
        ),
        Ordering::Greater => edit_distance(
            app_name[..app_name
                .char_indices()
                .map(|(i, _)| i)
                .nth(searched_app_name.len())
                .unwrap_or(app_name.len())]
                .to_lowercase()
                .as_ref(),
            searched_app_name.to_lowercase().as_ref(),
        ),
    }
}
