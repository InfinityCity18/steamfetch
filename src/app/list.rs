use json::{App, AppsList};

use crate::error::{ExitResult, IntoResultExitError};
use edit_distance::edit_distance;
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

    pub fn get_most_matching_app_id(&self, searched_app_name: &str, lang: &str) {
        let mut best_ref: Option<&App> = None;
        let mut best_match_value: usize = usize::MAX;
        let mut zero_edit_shortest: usize = usize::MAX;

        for app in &self.apps {}
    }
}

pub fn find_best_app_id(applist: AppsList, lang: &str, searched_app_name: &str) -> u32 {
    let applist = applist.applist.apps.app;

    let mut best_ref: Option<&App> = None;
    let mut best_match_value: usize = usize::MAX;
    let mut zero_edit_shortest: usize = usize::MAX;

    for app in &applist {
        if app.name.to_lowercase() == searched_app_name.to_lowercase() {
            match crate::appinfo_query::get_app_info(app.appid, lang) {
                Some(_) => return app.appid,
                None => continue,
            }
        }

        let edit = match &app
            .name
            .len()
            .partial_cmp(&searched_app_name.len())
            .expect("lengths should be comparable")
        {
            Ordering::Less => edit_distance(
                &app.name.to_lowercase(),
                searched_app_name.to_lowercase().as_ref(),
            ),
            Ordering::Equal => edit_distance(
                &app.name.to_lowercase(),
                searched_app_name.to_lowercase().as_ref(),
            ),
            Ordering::Greater => edit_distance(
                &app.name[..app
                    .name
                    .char_indices()
                    .map(|(i, _)| i)
                    .nth(searched_app_name.len())
                    .unwrap_or(app.name.len())]
                    .to_lowercase(),
                searched_app_name.to_lowercase().as_ref(),
            ),
        };

        if edit == 0 && app.name.len() <= zero_edit_shortest {
            zero_edit_shortest = app.name.len();
            best_ref = Some(app);
            best_match_value = edit;
        } else if edit < best_match_value {
            best_ref = Some(app);
            best_match_value = edit;
        }
    }
    if let Some(best_ref) = best_ref {
        println!("{:#?}", best_ref);
        return best_ref.appid;
    } else {
        error_and_quit(format!("applist was empty").as_ref());
    }
}

fn get_edit_distance(app_name: &str, searched_app_name: &str) -> usize {
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
