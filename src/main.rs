#![allow(dead_code)]

use glyphs::FancyFont;

mod app;
mod arg;
mod error;
mod glyphs;
mod print;

fn main() {
    let width = 48;
    let _height = (width * 10) / 46;
    let _offset = 0;
    let fg_mod = print::constants::TEXT_COLOR.to_string() + print::constants::BOLD;
    let bg_color = print::constants::NONE;

    app::print::<FancyFont>(
        "terraria", "english", width, _height, _offset, &fg_mod, bg_color,
    )
    .unwrap();
}
