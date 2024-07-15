pub trait Glyph {
    const LEFT_TOP_CORNER: &'static str;
    const LEFT_BOT_CORNER: &'static str;
    const RIGHT_TOP_CORNER: &'static str;
    const RIGHT_BOT_CORNER: &'static str;
    const BAR: &'static str;
    const PIPE: &'static str;
    const LEFT_T: &'static str;
    const RIGHT_T: &'static str;
}

pub struct NoFancyFont;
pub struct FancyFont;

impl Glyph for NoFancyFont {
    const LEFT_TOP_CORNER: &'static str = "#";
    const LEFT_BOT_CORNER: &'static str = "#";
    const RIGHT_TOP_CORNER: &'static str = "#";
    const RIGHT_BOT_CORNER: &'static str = "#";
    const BAR: &'static str = "-";
    const PIPE: &'static str = "|";
    const LEFT_T: &'static str = "#";
    const RIGHT_T: &'static str = "#";
}

impl Glyph for FancyFont {
    const LEFT_TOP_CORNER: &'static str = "╭";
    const LEFT_BOT_CORNER: &'static str = "╰";
    const RIGHT_TOP_CORNER: &'static str = "╮";
    const RIGHT_BOT_CORNER: &'static str = "╯";
    const BAR: &'static str = "─";
    const PIPE: &'static str = "│";
    const LEFT_T: &'static str = "⎬";
    const RIGHT_T: &'static str = "⎨";
}
