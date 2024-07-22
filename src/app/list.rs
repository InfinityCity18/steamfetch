use json::App;

use rayon::prelude::*;

pub use json::AppsList;

use crate::{
    error::{ExitError, ExitResult, IntoResultExitError},
    print,
};
use serde_json::Value;
use std::{
    cmp::Ordering,
    sync::{Arc, Mutex},
};

mod json;

impl AppsList {
    pub fn get_applist() -> ExitResult<'static, AppsList> {
        let url = super::links::APP_LIST;
        let response = reqwest::blocking::get(url).into_exit_error("fetching applist failed")?;

        let mut list: Value = response.json().into_exit_error("parsing json failed")?;
        serde_json::from_value(list["applist"]["apps"].take())
            .into_exit_error("parsing json failed")
    }

    pub fn get_most_matching_app_id(
        &self,
        searched_app_name: &str,
        lang: &str,
    ) -> ExitResult<'static, u32> {
        let best_match_value_lock: Arc<Mutex<usize>> = Arc::new(Mutex::new(usize::MAX));
        let zero_edit_shortest_lock: Arc<Mutex<usize>> = Arc::new(Mutex::new(usize::MAX));
        let best_matches_lock: Arc<Mutex<Vec<&App>>> = Arc::new(Mutex::new(Vec::new()));

        self.apps.par_iter().any(|app| {
            if app.name.to_lowercase() == searched_app_name {
                match super::info::AppInfoRoot::get_app_info(app.appid, lang) {
                    Ok(_) => {
                        let mut best_matches = best_matches_lock.lock().unwrap();
                        best_matches.clear();
                        best_matches.push(app);
                        return true;
                    }
                    Err(_) => return false,
                }
            }
            let edit = get_edit_distance(&app.name, &searched_app_name);
            let mut zero_edit_shortest = zero_edit_shortest_lock.lock().unwrap();
            let mut best_match_value = best_match_value_lock.lock().unwrap();
            let mut best_matches = best_matches_lock.lock().unwrap();

            if edit == 0 && app.name.len() < *zero_edit_shortest {
                *zero_edit_shortest = app.name.len();
                *best_match_value = edit;
                best_matches.clear();
                best_matches.push(app);
            } else if edit == 0 && app.name.len() == *zero_edit_shortest {
                *best_match_value = edit;
                best_matches.push(app);
            } else if edit < *best_match_value {
                *best_match_value = edit;
                best_matches.clear();
                best_matches.push(app);
            } else if edit == *best_match_value {
                best_matches.push(app);
            }
            false
        });
        let best_matches = Arc::clone(&best_matches_lock);
        for app in &*best_matches.lock().unwrap() {
            match super::info::AppInfoRoot::get_app_info(app.appid, lang) {
                Ok(_) => {
                    return Ok(app.appid);
                }
                Err(_) => continue,
            }
        }
        return Err(ExitError("no such app found"));
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
