use crate::applist_json::{App, AppListRoot};
use crate::error::error_and_quit;

fn get_applist(url: &str) -> AppListRoot {
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
        let edit = edit_distance::edit_distance(&app.name, searched_app_name);
        if edit <= best_match_value {
            best_ref = Some(app);
            best_match_value = edit;
        }
    }
    if let Some(best_ref) = best_ref {
        return best_ref.appid;
    } else {
        error_and_quit(format!("applist was empty").as_ref());
    }
}
