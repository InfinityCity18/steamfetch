#![allow(dead_code)]

use print::Character;

mod app;
mod arg;
mod error;
mod glyphs;
mod print;

fn main() {
    let width = 48;
    let _height = (width * 10) / 46;
    let _offset = 0;
    let _h = Character::create_vec_from_str("wdad", "w", "w");
}
