use crate::error::ExitResult;

use super::line::Line;

pub struct Module<'a> {
    pub lines: Vec<Line<'a>>,
}

impl<'a> Module<'a> {
    pub fn print(&self) -> ExitResult<()> {
        for line in &self.lines {
            line.print()?;
        }
        Ok(())
    }

    pub fn print_image(
        url: &str,
        width: u16,
        height: u16,
        x_offset: u16,
        y_offset: i16,
    ) -> ExitResult<()> {
        super::image::print_image(url, width, height, x_offset, y_offset)?;
        Ok(())
    }
}
