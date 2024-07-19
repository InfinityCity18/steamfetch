use std::collections::VecDeque;

use super::character::Character;
use crate::error::ExitResult;

#[derive(Clone)]
pub struct Line<'a> {
    content: Vec<Character<'a>>,
    whitespace_offset: u16,
}

impl<'a> Line<'a> {
    pub(super) fn print(&self) -> ExitResult<'static, ()> {
        print!("{}", " ".repeat(self.whitespace_offset.into()));
        for c in &self.content {
            c.print()?;
        }
        print!("\n");
        Ok(())
    }

    pub fn border_and_filling(
        width: u16,
        border_left: Character<'a>,
        border_right: Character<'a>,
        filling: Character<'a>,
        whitespace_offset: u16,
    ) -> Self {
        let mut content: Vec<Character<'a>> = Vec::new();
        content.push(border_left);
        for _ in 1..=width {
            content.push(filling);
        }
        content.push(border_right);
        Self {
            content,
            whitespace_offset,
        }
    }

    pub fn border_and_filling_with_centered_text(
        width: u16,
        border_left: Character<'a>,
        border_right: Character<'a>,
        filling: Character<'a>,
        whitespace_offset: u16,
        text: Vec<Character<'a>>,
    ) -> Self {
        let mut content: Vec<Character<'a>> = Vec::new();

        content.push(border_left);

        let left = (width as usize - text.len()) / 2;
        let right = width as usize - left - text.len();
        for _ in 1..=left {
            content.push(filling);
        }
        for mut c in text {
            if c.get_char() == ' ' {
                c.set_char(filling.get_char());
            }
            content.push(c);
        }

        for _ in 1..=right {
            content.push(filling);
        }

        content.push(border_right);

        Self {
            content,
            whitespace_offset,
        }
    }

    pub fn border_filling_wrapping_text(
        width: u16,
        border_left: Character<'a>,
        border_right: Character<'a>,
        filling: Character<'a>,
        whitespace_offset: u16,
        text: Vec<Character<'a>>,
    ) -> Vec<Self> {
        let mut text: VecDeque<Character> = text.into();
        let mut lines: Vec<Line> = Vec::new();

        let space_for_text = width - 2;

        while !text.is_empty() {
            let mut content: Vec<Character> = Vec::new();
            content.push(border_left);
            content.push(filling);

            for _ in 1..=space_for_text {
                if let Some(c) = text.pop_front() {
                    content.push(c);
                } else {
                    content.push(filling);
                }
            }
            content.push(filling);
            content.push(border_right);

            lines.push(Self {
                content,
                whitespace_offset,
            })
        }

        lines
    }
}
