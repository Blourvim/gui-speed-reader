use std::u8;

#[derive(Copy, Clone)]
struct LetterBox {
    sizex: f32,
    sizey: f32,
    content: char,
}

impl LetterBox {
    fn new(sizex: f32, sizey: f32, content: char) -> Self {
        Self {
            sizex,
            sizey,
            content,
        }
    }
}

impl LetterBox {
    fn change_char(&mut self, new_char: char) {
        if self.content != new_char {
            self.content = new_char
        }
    }
}

struct Viewport {
    letter_boxes: [LetterBox; 26], // Fixed-size array of 26 LetterBox instances
}
impl Viewport {
    fn new(word: &str) -> Self {
        let sizex = 10.0;
        let sizey = 15.0;

        // Create an array to hold the 26 LetterBox instances
        let mut boxes: [LetterBox; 26] = [LetterBox::new(sizex, sizey, ' '); 26];

        Self {
            letter_boxes: boxes,
        }
    }

    fn update(mut self, word: &str) -> Self {
        let chars: Vec<char> = word.chars().collect();
        let word_len: i32 = chars.len().try_into().unwrap();

        let start_index = 13usize - ((word_len / 2) as f32).floor() as usize;

        for (i, c) in word.chars().take(26).enumerate() {
            self.letter_boxes[start_index + i].change_char(c)
        }
        self
    }
}
