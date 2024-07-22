mod info;
mod links;
mod list;
mod print;
mod reviews;

use crate::{error::ExitResult, glyphs::Glyph, print::Module};

pub fn print<T: Glyph>(
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

    let app_list = list::AppsList::get_applist()?;
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    let app_id = app_list.get_most_matching_app_id(app_name, lang)?;
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    let app = info::AppInfoRoot::get_app_info(app_id, lang)?;
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    let review = reviews::QuerySummary::get_app_reviews(app_id)?;
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    let player_count = info::AppInfoRoot::get_player_count(app_id)?;
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
