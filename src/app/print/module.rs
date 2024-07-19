use crate::glyphs::Glyph;

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
        width: u16,
        fg_mod: &'a str,
        bg_color: &'a str,
        whitespace_offset: u16,
    ) -> Self {
        let mut lines: Vec<Line> = Vec::new();
    }
}
