use self::constants::*;
use self::module::Module;
use crate::appinfo_json::AppInfoRoot;
use crate::glyphs::{FancyFont, Glyph};

mod module;

pub fn print_app_info<T: Glyph>(app: AppInfoRoot, width: u16, height: u16, offset: u16) {
    let app = app.data;
    let fg_mod = BOLD.to_string() + TEXT_COLOR;
    let bg_color = "";
    let frame = Module::frame::<FancyFont>(&app.name, width, height, &fg_mod, bg_color, offset);
    frame.print();
    Module::print_image(
        &app.header_image,
        width,
        height,
        offset,
        -(height as i16) - 1,
    );
}

/*
fn text_line(text: &str, width: u16, infill: &str) -> String {
    let mut output = String::new();

    let text = text.replace(" ", infill);
    let left = (width as usize - text.chars().count()) / 2;
    let right = width as usize - left - text.chars().count();

    for _ in 1..=left {
        output.push_str(infill);
    }
    output.push_str(&text);
    for _ in 1..=right {
        output.push_str(infill);
    }

    output
}

fn get_price_and_color(app: &AppInfo) -> (String, String) {
    if let Some(price_overview) = &app.price_overview {
        if price_overview.discount_percent > 0 {
            return (
                format!(
                    "{} -{}%",
                    price_overview.final_formatted, price_overview.discount_percent
                ),
                format!("{}{}", GREEN_BG, GREEN_TEXT),
            );
        } else {
            return (
                format!("{}", price_overview.final_formatted),
                BLUE_BG.to_string(),
            );
        }
    } else {
        return (String::from("Free"), format!("{}", BLUE_BG));
    }
}

fn get_reviews_and_color(reviews: &QuerySummary) -> (String, String) {
    if reviews.review_score_desc == "No user reviews" {
        return (String::from("No reviews"), GREY_BG.to_string());
    } else {
        return (
            format!(
                "{} : {}%",
                reviews.review_score_desc,
                ((reviews.total_positive as f64 / reviews.total_reviews as f64) * 100.0).round()
                    as u32
            ),
            BLUE_BG.to_string(),
        );
    }
}
*/
