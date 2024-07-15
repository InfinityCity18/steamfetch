use crate::appinfo_json::AppInfoRoot;
use crate::glyphs::Glyph;

const TEXT_COLOR: &str = "\x1b[36m";
const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";

pub fn print_app_info<T: Glyph>(app: AppInfoRoot, width: u16, height: u16) {
    let app = app.data;
    print!("{}{}", TEXT_COLOR, BOLD);
    print_image_frame::<T>(width, height, &app.name);
    crate::appimage::print_image(&app.header_image, width, height, -(height as i16) - 1);
}

fn print_horizontal_line(len: usize, bar: &str) {
    for _ in 1..=len {
        print!("{}", bar);
    }
}

fn print_horizontal_line_ended<T: Glyph>(left_end: &str, right_end: &str, bar: &str, len: usize) {
    print!("{}", left_end);
    print_horizontal_line(len, bar);
    print!("{}\n", right_end);
}

fn print_horizontal_line_ended_with_text<T: Glyph>(
    left_end: &str,
    right_end: &str,
    bar: &str,
    len: usize,
    text: &str,
) {
    let text = text.replace(" ", bar);
    print!("{}", left_end);
    let left = (len - text.chars().count()) / 2;
    let right = len - left - text.chars().count();
    for _ in 1..=left {
        print!("{}", bar);
    }
    print!("{}", text);
    for _ in 1..=right {
        print!("{}", bar);
    }
    print!("{}\n", right_end);
}

fn print_image_frame<T: Glyph>(width: u16, height: u16, app_name: &str) {
    print_horizontal_line_ended_with_text::<T>(
        T::LEFT_TOP_CORNER,
        T::RIGHT_TOP_CORNER,
        T::BAR,
        width.into(),
        "steamfetch",
    );
    for _ in 1..=height {
        print_horizontal_line_ended::<T>(T::PIPE, T::PIPE, " ", width.into());
    }
    print_horizontal_line_ended_with_text::<T>(
        T::LEFT_T,
        T::RIGHT_T,
        T::BAR,
        width.into(),
        app_name,
    );
}
