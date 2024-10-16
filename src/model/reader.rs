use std::env;
use super::data::{DisplayArray, Message};

pub struct Reader {
    pub words: Vec<String>,
    pub index: usize,
    pub display_array: DisplayArray,
}

// we do that here since the iced calls default when istantiating the Reader
impl Default for Reader {
    fn default() -> Self {
        Self::new()
    }
}

impl Reader {
    pub fn new() -> Self {
        let args: Vec<String> = env::args().collect();
        // since the clipboard implementation  might be system dependent, i chose to implement the
        // input text as a argument, so that it can be piped into the reader in the unix fashion
        let text = &args[1];
        let words: Vec<String> = text.split_whitespace().map(String::from).collect();
        Self {
            display_array: DisplayArray::new(),
            words: words.clone(),
            index: 0usize,
        }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Increment => {
                self.display_array.update(&self.words[self.index]);
                self.index = self.index + 1
            }
        }
    }
}
