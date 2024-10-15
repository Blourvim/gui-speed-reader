const DISPLAY_ARRAY_LEN: usize = 27;

#[derive(Copy, Clone)]
pub struct Rectangle {
    pub content: char,
}

#[derive(Debug, Clone)]
pub enum Message {
    Increment,
}
pub struct DisplayArray {
    pub displays: [Rectangle; DISPLAY_ARRAY_LEN],
}

impl Rectangle {
    pub fn new(content: char) -> Self {
        Self { content }
    }

    pub fn change_char(&mut self, new_char: char) {
        if self.content != new_char {
            self.content = new_char
        }
    }

    pub fn display(&self) -> char {
        self.content
    }
}

impl DisplayArray {
    pub fn new() -> Self {
        let boxes: [Rectangle; DISPLAY_ARRAY_LEN] = [Rectangle::new(' '); DISPLAY_ARRAY_LEN];
        Self { displays: boxes }
    }

    fn clear_display(&mut self) {
        self.displays.iter_mut().for_each(|f| f.change_char(' '));
    }

    pub fn update(&mut self, new_word: &str) -> &Self {
        self.clear_display();
        let chars: Vec<char> = new_word.chars().collect();
        let word_len: i32 = chars.len().try_into().unwrap();

        let start_index = 13usize - ((word_len / 2) as f32).floor() as usize;

        for (i, c) in new_word.chars().take(DISPLAY_ARRAY_LEN).enumerate() {
            self.displays[start_index + i].change_char(c);
        }
        self
    }

    pub fn print(&self) {
        let display_string: String = self.displays.iter().map(|f| f.display()).collect();
        println!("{}", display_string);
    }
}
