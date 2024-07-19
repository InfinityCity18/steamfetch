use super::super::info::AppInfoRoot;
use super::super::reviews::QuerySummary;
use crate::glyphs::Glyph;
use crate::print::constants::*;

use crate::print::{Character, Line, Module};

impl<'a> Module<'a> {
    pub fn frame<T: Glyph>(
        appname: &str,
        width: u16,
        height: u16,
        fg_mod: &'a str,
        bg_color: &'a str,
        whitespace_offset: u16,
    ) -> Self {
        let mut lines: Vec<Line> = Vec::new();
        let left_top = Character::create(T::LEFT_TOP_CORNER, fg_mod, bg_color);
        let right_top = Character::create(T::RIGHT_TOP_CORNER, fg_mod, bg_color);
        let left_bot = Character::create(T::LEFT_T, fg_mod, bg_color);
        let right_bot = Character::create(T::RIGHT_T, fg_mod, bg_color);
        let bar = Character::create(T::BAR, fg_mod, bg_color);
        let pipe = Character::create(T::PIPE, fg_mod, bg_color);
        let whitespace = Character::create(' ', fg_mod, bg_color);

        let steamfetch = Character::create_vec_from_str("steamfetch", fg_mod, bg_color);
        let appname = Character::create_vec_from_str(appname, fg_mod, bg_color);

        let bar_line = Line::border_and_filling(width, pipe, pipe, whitespace, whitespace_offset);

        let steamfetch_line = Line::border_and_filling_with_centered_text(
            width,
            left_top,
            right_top,
            bar,
            whitespace_offset,
            steamfetch,
        );
        let appname_line = Line::border_and_filling_with_centered_text(
            width,
            left_bot,
            right_bot,
            bar,
            whitespace_offset,
            appname,
        );

        lines.push(steamfetch_line);
        for _ in 1..=height {
            lines.push(bar_line.clone());
        }
        lines.push(appname_line);

        Self { lines }
    }

    pub fn app_info<T: Glyph>(
        app: &AppInfoRoot,
        reviews: &QuerySummary,
        player_count: u32,
        width: u16,
        fg_mod: &'a str,
        bg_color: &'a str,
        whitespace_offset: u16,
    ) -> Self {
        let mut lines: Vec<Line> = Vec::new();

        let pipe = Character::create(T::PIPE, fg_mod, bg_color);
        let whitespace = Character::create(' ', fg_mod, bg_color);
        let left_corner = Character::create(T::LEFT_BOT_CORNER, fg_mod, bg_color);
        let right_corner = Character::create(T::RIGHT_BOT_CORNER, fg_mod, bg_color);
        let bar = Character::create(T::BAR, fg_mod, bg_color);

        let mut price = Character::create_vec_from_str("Price", fg_mod, bg_color);
        price.append(&mut Character::create_vec_from_str(": ", NONE, NONE));
        if app.data.is_free {
            price.push(Character::create(T::LEFT_HALF_CIRCLE, BLUE_BG, NONE));
            price.append(&mut Character::create_vec_from_str("Free", NONE, BLUE_BG));
            price.push(Character::create(T::RIGHT_HALF_CIRCLE, BLUE_BG, NONE));
        } else if app.data.price_overview.as_ref().unwrap().discount_percent == 0 {
            price.append(&mut Character::create_vec_from_str(
                &app.data.price_overview.as_ref().unwrap().final_formatted,
                NONE,
                NONE,
            ));
        } else {
            price.push(Character::create(T::LEFT_HALF_CIRCLE, GREEN_DARK, NONE));
            price.append(&mut Character::create_vec_from_str(
                format!(
                    "{} at -{}%",
                    app.data.price_overview.as_ref().unwrap().final_formatted,
                    app.data.price_overview.as_ref().unwrap().discount_percent
                )
                .as_ref(),
                GREEN_LIGHT,
                GREEN_DARK,
            ));
            price.push(Character::create(T::RIGHT_HALF_CIRCLE, GREEN_DARK, NONE));
        }
        Line::border_filling_wrapping_text(width, pipe, pipe, whitespace, whitespace_offset, price)
            .into_iter()
            .for_each(|l| lines.push(l));

        let mut id = Character::create_vec_from_str("ID", fg_mod, bg_color);
        id.append(&mut Character::create_vec_from_str(
            format!(": {}", app.data.steam_appid).as_ref(),
            NONE,
            NONE,
        ));
        Line::border_filling_wrapping_text(width, pipe, pipe, whitespace, whitespace_offset, id)
            .into_iter()
            .for_each(|l| lines.push(l));

        let mut review = Character::create_vec_from_str("Reviews", fg_mod, bg_color);
        review.append(&mut Character::create_vec_from_str(": ", NONE, NONE));
        if reviews.review_score_desc == "No user reviews" {
            review.push(Character::create(T::LEFT_HALF_CIRCLE, GREY_FG, NONE));
            review.append(&mut Character::create_vec_from_str("Free", NONE, GREY_BG));
            review.push(Character::create(T::RIGHT_HALF_CIRCLE, GREY_FG, NONE));
        } else {
            let percentage = ((reviews.total_positive as f64 / reviews.total_reviews as f64)
                * 100.0)
                .round() as u32;
            review.push(Character::create(T::LEFT_HALF_CIRCLE, BLUE_FG, NONE));
            review.append(&mut Character::create_vec_from_str(
                format!(
                    "{}, {}% ({})",
                    reviews.review_score_desc, percentage, reviews.total_reviews
                )
                .as_ref(),
                NONE,
                BLUE_BG,
            ));
            review.push(Character::create(T::RIGHT_HALF_CIRCLE, BLUE_FG, NONE));
        }
        Line::border_filling_wrapping_text(
            width,
            pipe,
            pipe,
            whitespace,
            whitespace_offset,
            review,
        )
        .into_iter()
        .for_each(|l| lines.push(l));

        let mut release = Character::create_vec_from_str("Release date", fg_mod, bg_color);
        release.append(&mut Character::create_vec_from_str(
            format!(": {}", app.data.release_date.date).as_ref(),
            NONE,
            NONE,
        ));
        Line::border_filling_wrapping_text(
            width,
            pipe,
            pipe,
            whitespace,
            whitespace_offset,
            release,
        )
        .into_iter()
        .for_each(|l| lines.push(l));

        let mut players_cnt = Character::create_vec_from_str("Player count", fg_mod, bg_color);
        players_cnt.append(&mut Character::create_vec_from_str(
            format!(": {}", player_count).as_ref(),
            NONE,
            NONE,
        ));
        Line::border_filling_wrapping_text(
            width,
            pipe,
            pipe,
            whitespace,
            whitespace_offset,
            players_cnt,
        )
        .into_iter()
        .for_each(|l| lines.push(l));

        let mut developer = Character::create_vec_from_str("Developer", fg_mod, bg_color);
        developer.append(&mut Character::create_vec_from_str(
            format!(": {}", app.data.developers[0]).as_ref(),
            NONE,
            NONE,
        ));
        Line::border_filling_wrapping_text(
            width,
            pipe,
            pipe,
            whitespace,
            whitespace_offset,
            developer,
        )
        .into_iter()
        .for_each(|l| lines.push(l));

        let mut publisher = Character::create_vec_from_str("Publisher", fg_mod, bg_color);
        publisher.append(&mut Character::create_vec_from_str(
            format!(": {}", app.data.publishers[0]).as_ref(),
            NONE,
            NONE,
        ));
        Line::border_filling_wrapping_text(
            width,
            pipe,
            pipe,
            whitespace,
            whitespace_offset,
            publisher,
        )
        .into_iter()
        .for_each(|l| lines.push(l));

        let mut platforms = Character::create_vec_from_str("Platforms", fg_mod, bg_color);
        let mut platforms_str = String::new();
        if app.data.platforms.linux {
            platforms_str += &T::LINUX.to_string();
            platforms_str += " ";
        }
        if app.data.platforms.mac {
            platforms_str += &T::MACOS.to_string();
            platforms_str += " ";
        }
        if app.data.platforms.windows {
            platforms_str += &T::WINDOWS.to_string();
            platforms_str += " ";
        }
        platforms.append(&mut Character::create_vec_from_str(
            format!(": {}", platforms_str).as_ref(),
            NONE,
            NONE,
        ));
        Line::border_filling_wrapping_text(
            width,
            pipe,
            pipe,
            whitespace,
            whitespace_offset,
            platforms,
        )
        .into_iter()
        .for_each(|l| lines.push(l));

        let mut description = Character::create_vec_from_str("Description", fg_mod, bg_color);
        description.append(&mut Character::create_vec_from_str(
            format!(": {}", app.data.short_description).as_ref(),
            NONE,
            NONE,
        ));
        Line::border_filling_wrapping_text(
            width,
            pipe,
            pipe,
            whitespace,
            whitespace_offset,
            description,
        )
        .into_iter()
        .for_each(|l| lines.push(l));

        lines.push(Line::border_and_filling(
            width,
            left_corner,
            right_corner,
            bar,
            whitespace_offset,
        ));

        Self { lines }
    }
}
