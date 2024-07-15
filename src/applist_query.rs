use std::cmp::Ordering;

use edit_distance::edit_distance;

use crate::applist_json::{App, AppListRoot};
use crate::error::error_and_quit;

pub fn get_applist(url: &str) -> AppListRoot {
    let response = reqwest::blocking::get(url)
        .unwrap_or_else(|_| error_and_quit(format!("failed to get data from \"{url}\"").as_ref()));

    response
        .json()
        .unwrap_or_else(|_| error_and_quit(format!("parsing json failed").as_ref()))
}

pub fn find_best_app_id(applist: AppListRoot, searched_app_name: &str) -> u32 {
    let applist = applist.applist.apps.app;

    let mut best_ref: Option<&App> = None;
    let mut best_match_value: usize = usize::MAX;

    for app in &applist {
        println!(
            "searched_app_name: {searched_app_name}, app name : {}",
            app.name
        );
        println!(
            "wierd shit: {}",
            &app.name[..app
                .name
                .char_indices()
                .map(|(i, _)| i)
                .nth(searched_app_name.len())
                .unwrap_or(app.name.len())]
        );
        println!(
            "weird shit 2: {}",
            &searched_app_name[..searched_app_name
                .char_indices()
                .map(|(i, _)| i)
                .nth(app.name.len())
                .unwrap_or(searched_app_name.len())],
        );

        if app.name.to_lowercase() == searched_app_name.to_lowercase() {
            match crate::appinfo_query::get_app_info(app.appid) {
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
        if edit <= best_match_value {
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
