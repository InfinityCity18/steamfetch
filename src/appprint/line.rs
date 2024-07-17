use crate::appprint::character::Character;

pub struct Line {
    content: Vec<Character>,
    whitespace_offset: u16,
}

impl Line {
    pub fn print(&self) {
        print!("{}", " ".repeat(self.whitespace_offset.into()));
        self.content.iter().for_each(|c| c.print());
        print!("\n");
    }

    pub fn border_and_filling(
        width: u16,
        border_left: Character,
        border_right: Character,
        filling: Character,
        whitespace_offset: u16,
    ) -> Self {
        let mut content: Vec<Character> = Vec::new();
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
        border_left: Character,
        border_right: Character,
        filling: Character,
        whitespace_offset: u16,
        text: Vec<Character>,
    ) -> Self {
        let mut content: Vec<Character> = Vec::new();
        content.push(border_left);
        let left = (width as usize - text.len()) / 2;
        for mut c in text {
            if c.get_char() == ' ' {
                c.set_char(filling.get_char());
            }
            content.push(c);
        }
        content.push(border_right);

        Self {
            content,
            whitespace_offset,
        }
    }
}
