use crate::appinfo_json::{AppInfo, AppInfoRoot};
use crate::appreviews_json::QuerySummary;
use crate::appreviews_query::get_app_reviews;
use crate::glyphs::{FancyFont, Glyph};

const TEXT_COLOR: &str = "\x1b[36m";
const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const GREEN_BG: &str = "\x1b[42m";
const GREEN_TEXT: &str = "\x1b[92m";
const BLUE_BG: &str = "\x1b[44m";
const GREY_BG: &str = "\x1b[100m";

struct Module {
    lines: Vec<Line>,
}

struct Line {
    outerleft: &'static str,
    outerright: &'static str,
    inside: String,
    offset: u16,
}

impl Module {
    pub fn print(&self) {
        self.lines.iter().for_each(|line| line.print());
    }

    pub fn escape_code(code: &str) {
        print!("{}", code);
    }

    pub fn build_image_frame<T: Glyph>(
        app_name: &str,
        width: u16,
        height: u16,
        offset: u16,
    ) -> Self {
        let mut lines: Vec<Line> = Vec::new();
        lines.push(Line {
            outerleft: T::LEFT_TOP_CORNER,
            outerright: T::RIGHT_TOP_CORNER,
            inside: text_line("steamfetch", width, T::BAR),
            offset,
        });
        for _ in 1..=height {
            lines.push(Line {
                outerleft: T::PIPE,
                outerright: T::PIPE,
                inside: String::from(" ".repeat(width.into())),
                offset,
            })
        }
        lines.push(Line {
            outerleft: T::LEFT_T,
            outerright: T::RIGHT_T,
            inside: text_line(app_name, width, T::BAR),
            offset,
        });

        Self { lines }
    }

    pub fn build_price_reviews_mod<T: Glyph>(app: &AppInfo, width: u16, offset: u16) -> Self {
        let mut lines: Vec<Line> = Vec::new();
        let reviews: QuerySummary = get_app_reviews(app.steam_appid);
        let (review, review_color) = get_reviews_and_color(&reviews);
        let (price, price_color) = get_price_and_color(app);

        let price_len = price.chars().count();
        let review_len = review.chars().count();
        let diff = width as usize - price_len - review_len - 6;

        lines.push(Line {
            outerleft: T::PIPE,
            outerright: T::PIPE,
            inside: String::from(" ".repeat(width.into())),
            offset,
        });

        lines.push(Line {
            outerleft: T::PIPE,
            outerright: T::PIPE,
            inside: format!(
                " {}{}{}{}{}{}{} ",
                price_color,
                " ".repeat(price_len + 2),
                RESET,
                " ".repeat(diff),
                review_color,
                " ".repeat(review_len + 2),
                RESET
            ),
            offset,
        });

        lines.push(Line {
            outerleft: T::PIPE,
            outerright: T::PIPE,
            inside: format!(
                " {} {} {}{}{} {} {} ",
                price_color,
                price,
                RESET,
                " ".repeat(diff),
                review_color,
                review,
                RESET
            ),
            offset,
        });

        lines.push(Line {
            outerleft: T::PIPE,
            outerright: T::PIPE,
            inside: format!(
                " {}{}{}{}{}{}{} ",
                price_color,
                " ".repeat(price_len + 2),
                RESET,
                " ".repeat(diff),
                review_color,
                " ".repeat(review_len + 2),
                RESET
            ),
            offset,
        });

        Self { lines }
    }

    pub fn print_image(url: &str, width: u16, height: u16, x_offset: u16, y_offset: i16) {
        crate::appimage::print_image(url, width, height, x_offset, y_offset);
    }
}

impl Line {
    fn print(&self) {
        print!("{}", " ".repeat(self.offset.into()));
        print!("{}{}{}\n", self.outerleft, self.inside, self.outerright);
    }
}

pub fn print_app_info<T: Glyph>(app: AppInfoRoot, width: u16, height: u16, offset: u16) {
    let app = app.data;
    print!("{}{}", TEXT_COLOR, BOLD);

    let frame = Module::build_image_frame::<FancyFont>(&app.name, width, height, offset);
    frame.print();
    Module::print_image(
        &app.header_image,
        width,
        height,
        offset,
        -(height as i16) - 1,
    );
    let price_and_reviews = Module::build_price_reviews_mod::<FancyFont>(&app, width, offset);
    price_and_reviews.print();
}

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
