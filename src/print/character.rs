use super::constants::RESET;
use std::io::Write;

#[derive(Clone, Copy)]
pub struct Character<'a> {
    content: char,
    fg_mod: &'a str,
    bg_color: &'a str,
}

impl<'a> Character<'a> {
    pub fn print(&self) {
        let temp = self.content.to_string();
        let content = vec![self.fg_mod, self.bg_color, &temp, RESET];
        let mut out = std::io::stdout();
        for x in content {
            out.write(x.as_bytes());
        }
    }

    pub fn create(content: char, fg_mod: &'a str, bg_color: &'a str) -> Self {
        Self {
            content,
            fg_mod,
            bg_color,
        }
    }

    pub fn create_vec_from_str(text: &str, fg_mod: &'a str, bg_color: &'a str) -> Vec<Self> {
        let mut vec: Vec<Self> = Vec::new();
        for c in text.chars() {
            vec.push(Self {
                content: c,
                fg_mod,
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
