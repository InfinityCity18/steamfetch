use crate::appprint::line::Line;

struct Module {
    lines: Vec<Line>,
}

impl Module {
    pub fn print(&self) {
        self.lines.iter().for_each(|line| line.print());
    }

    pub fn print_image(url: &str, width: u16, height: u16, x_offset: u16, y_offset: i16) {
        crate::appimage::print_image(url, width, height, x_offset, y_offset);
    }
}
