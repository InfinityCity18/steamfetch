pub fn find_best_app_id(applist: AppListRoot, lang: &str, searched_app_name: &str) -> u32 {
    let applist = applist.applist.apps.app;

    let mut best_ref: Option<&App> = None;
    let mut best_match_value: usize = usize::MAX;
    let mut zero_edit_shortest: usize = usize::MAX;

    for app in &applist {
        println!(
            "searched_app_name: {searched_app_name}, app name : {}",
            app.name
        );
        println!(
            "wierd thing: {}",
            &app.name[..app
                .name
                .char_indices()
                .map(|(i, _)| i)
                .nth(searched_app_name.len())
                .unwrap_or(app.name.len())]
        );
        println!(
            "weird thing 2: {}",
            &searched_app_name[..searched_app_name
                .char_indices()
                .map(|(i, _)| i)
                .nth(app.name.len())
                .unwrap_or(searched_app_name.len())],
        );

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
