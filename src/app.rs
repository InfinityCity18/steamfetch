mod info;
mod links;
mod list;
mod print;
mod reviews;

use crate::{
    error::{ExitResult, IntoResultExitError},
    glyphs::Glyph,
    print::Module,
};

pub async fn print<T: Glyph>(
    app_name: &str,
    lang: &str,
    width: u16,
    frame_height: u16,
    whitespace_offset: u16,
    fg_mod: &str,
    bg_color: &str,
) -> ExitResult<'static, ()> {
    use std::time::Instant;
    let now = Instant::now();

    let app_list = list::Applist::get_applist().await?;
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    let app_id = app_list.get_most_matching_app_id(app_name, lang).await?;
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    let lang_mv = lang.to_string();
    let app = tokio::spawn(async move {
        let lang_mv = lang_mv;
        info::AppInfoRoot::get_app_info(app_id, &lang_mv).await
    });
    let review = tokio::spawn(reviews::QuerySummary::get_app_reviews(app_id));
    let player_count = tokio::spawn(info::AppInfoRoot::get_player_count(app_id));
    let app = app.await.into_exit_error("fail")??;
    let review = review.await.into_exit_error("fail")??;
    let player_count = player_count.await.into_exit_error("fail")??;

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    let frame = Module::frame::<T>(
        &app.data.name,
        width,
        frame_height,
        fg_mod,
        bg_color,
        whitespace_offset,
    );
    let app_info = Module::app_info::<T>(
        &app,
        &review,
        player_count,
        width,
        fg_mod,
        bg_color,
        whitespace_offset,
    );
    frame.print()?;
    Module::print_image(
        &app.data.header_image,
        width,
        frame_height,
        whitespace_offset + 1,
        -(frame_height as i16) - 1,
    )?;
    app_info.print()?;

    Ok(())
}
