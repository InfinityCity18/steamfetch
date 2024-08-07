pub trait Glyph {
    const LEFT_TOP_CORNER: char;
    const LEFT_BOT_CORNER: char;
    const RIGHT_TOP_CORNER: char;
    const RIGHT_BOT_CORNER: char;
    const BAR: char;
    const PIPE: char;
    const LEFT_T: char;
    const RIGHT_T: char;
    const LEFT_HALF_CIRCLE: char;
    const RIGHT_HALF_CIRCLE: char;
    const LINUX: char;
    const MACOS: char;
    const WINDOWS: char;
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
    const LEFT_HALF_CIRCLE: char = ' ';
    const RIGHT_HALF_CIRCLE: char = ' ';
    const LINUX: char = 'L';
    const MACOS: char = 'M';
    const WINDOWS: char = 'W';
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
    const LEFT_HALF_CIRCLE: char = '';
    const RIGHT_HALF_CIRCLE: char = '';
    const LINUX: char = '';
    const MACOS: char = '';
    const WINDOWS: char = '';
}
