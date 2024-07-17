pub trait Glyph {
    const LEFT_TOP_CORNER: char;
    const LEFT_BOT_CORNER: char;
    const RIGHT_TOP_CORNER: char;
    const RIGHT_BOT_CORNER: char;
    const BAR: char;
    const PIPE: char;
    const LEFT_T: char;
    const RIGHT_T: char;
}

pub struct NoFancyFont;
pub struct FancyFont;

impl Glyph for NoFancyFont {
    const LEFT_TOP_CORNER: char = '#';
    const LEFT_BOT_CORNER: char = '#';
    const RIGHT_TOP_CORNER: char = '#';
    const RIGHT_BOT_CORNER: char = '#';
    const BAR: char = '-';
    const PIPE: char = '|';
    const LEFT_T: char = '#';
    const RIGHT_T: char = '#';
}

impl Glyph for FancyFont {
    const LEFT_TOP_CORNER: char = '╭';
    const LEFT_BOT_CORNER: char = '╰';
    const RIGHT_TOP_CORNER: char = '╮';
    const RIGHT_BOT_CORNER: char = '╯';
    const BAR: char = '─';
    const PIPE: char = '│';
    const LEFT_T: char = '├';
    const RIGHT_T: char = '┤';
}
