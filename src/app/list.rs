use json::App;

use rayon::prelude::*;

pub use json::Applist;
use tokio::runtime::Handle;

use crate::error::{ExitError, ExitResult, IntoResultExitError};
use serde_json::Value;
use std::{
    cmp::Ordering,
    sync::{Arc, Mutex},
};

mod json;

impl Applist {
    pub async fn get_applist() -> ExitResult<'static, Applist> {
        let url = super::links::APP_LIST;
        let response = reqwest::get(url)
            .await
            .into_exit_error("fetching applist failed")?;

        let mut list: Value = response
            .json()
            .await
            .into_exit_error("parsing json failed")?;
        serde_json::from_value(list["applist"].take()).into_exit_error("parsing json failed")
    }

    pub async fn get_most_matching_app_id(
        &self,
        searched_app_name: &str,
        lang: &str,
    ) -> ExitResult<'static, u32> {
        let best_match_value_lock: Arc<Mutex<usize>> = Arc::new(Mutex::new(usize::MAX));
        let zero_edit_shortest_lock: Arc<Mutex<usize>> = Arc::new(Mutex::new(usize::MAX));
        let best_matches_lock: Arc<Mutex<Vec<&App>>> = Arc::new(Mutex::new(Vec::new()));

        let handle = Handle::current();

        self.apps.par_iter().any(|app| {
            if app.name.to_lowercase() == searched_app_name {
                match handle.block_on(super::info::AppInfoRoot::get_app_info(app.appid, lang)) {
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
        let mut best_matches = best_matches.lock().unwrap();
        best_matches.sort_by(|a: &&App, b: &&App| a.name.len().cmp(&b.name.len()));

        for app in &*best_matches {
            match super::info::AppInfoRoot::get_app_info(app.appid, lang).await {
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
