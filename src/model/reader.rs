use super::{
    config::ReaderConfig,
    data::{DisplayArray, Message},
};
use std::{
    env::{self, args},
    path::Path,
};

pub struct Reader {
    pub words: Vec<String>,
    pub index: usize,
    pub display_array: DisplayArray,
    pub config: ReaderConfig,
}

// we do that here since the iced calls default when istantiating the Reader
impl Default for Reader {
    fn default() -> Self {
        Self::new()
    }
}

impl Reader {
    pub fn new() -> Self {
        let path = Path::new("./config.toml");
        let config: ReaderConfig = confy::load_path(path).unwrap();

        let args: Vec<String> = env::args().collect();

        let words: Vec<String> = if args.len() == 2 {
            args[1].split_whitespace().map(String::from).collect()
        } else {
            config
                .tutorial_text
                .split_whitespace()
                .map(String::from)
                .collect()
        };

        Self {
            display_array: DisplayArray::new(),
            words: words.clone(),
            index: 0usize,
            config,
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
