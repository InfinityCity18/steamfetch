use crate::appinfo_json::AppInfoRoot;
use crate::glyphs::Glyph;

const TEXT_COLOR: &str = "\x1b[36m";
const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";

pub fn print_app_info(app: AppInfoRoot, width: u16, height: u16) {
    crate::appimage::print_image(&app.data.header_image, width, height);
}

pub fn print_horizontal_line(len: usize, bar: &str) {
    for _ in 1..=len {
        print!("{}", bar);
    }
}

fn print_horizontal_line_ended<T: Glyph>(left_end: &str, right_end: &str, bar: &str, len: usize) {
    print!("{}", left_end);
    print_horizontal_line(len, bar);
    print!("{}", right_end);
}

fn print_image_frame<T: Glyph>(width: u16, height: u16) {
    print_horizontal_line_ended::<T>(
        T::LEFT_TOP_CORNER,
        T::RIGHT_TOP_CORNER,
        T::BAR,
        width.into(),
    );
    for _ in 1..=height {
        print_horizontal_line_ended::<T>(T::PIPE, T::PIPE, " ", width.into());
    }
    print_horizontal_line_ended::<T>(T::LEFT_T, T::RIGHT_T, T::BAR, width.into());
}
