use crate::appprint::constants::RESET;
use std::io::Write;

#[derive(Clone, Copy)]
pub struct Character {
    content: char,
    fg_color: &'static str,
    bg_color: &'static str,
}

impl Character {
    pub fn print(&self) {
        let temp = self.content.to_string();
        let content = vec![self.fg_color, self.bg_color, &temp, RESET];
        let mut out = std::io::stdout();
        for x in content {
            out.write(x.as_bytes());
        }
    }

    pub fn create_vec_from_str(
        text: &str,
        fg_color: &'static str,
        bg_color: &'static str,
    ) -> Vec<Self> {
        let mut vec: Vec<Self> = Vec::new();
        for c in text.chars() {
            vec.push(Self {
                content: c,
                fg_color,
                bg_color,
            });
        }
        vec
    }

    pub fn get_char(&self) -> char {
        self.content
    }

    pub fn set_char(&mut self, c: char) {
        self.content = c;
    }
}
