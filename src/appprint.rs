use crate::appinfo_json::AppInfoRoot;
use crate::glyphs::{FancyFont, Glyph};

const TEXT_COLOR: &str = "\x1b[36m";
const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const GREEN_BG: &str = "\x1b[42m";
const GREEN_TEXT: &str = "\x1b[92m";

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
