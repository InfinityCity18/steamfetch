use super::line::Line;

pub struct Module<'a> {
    lines: Vec<Line<'a>>,
}

impl<'a> Module<'a> {
    pub fn print(&self) {
        self.lines.iter().for_each(|line| line.print());
    }

    pub fn print_image(url: &str, width: u16, height: u16, x_offset: u16, y_offset: i16) {
        crate::appimage::print_image(url, width, height, x_offset, y_offset);
    }
}
