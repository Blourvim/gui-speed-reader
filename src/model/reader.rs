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
        // makes sense to extract this text from clipboard at this point
        let text = "Lorem ipsum dolor sit amet, officia excepteur ex fugiat reprehenderit enim labore culpa sint ad nisi Lorem pariatur mollit ex esse exercitation amet. Nisi anim cupidatat excepteur officia. Reprehenderit nostrud nostrud ipsum Lorem est aliquip amet voluptate voluptate dolor minim nulla est proident. Nostrud officia pariatur ut officia. Sit irure elit esse ea nulla sunt ex occaecat reprehenderit commodo officia dolor Lorem duis laboris cupidatat officia voluptate. Culpa proident adipisicing id nulla nisi laboris ex in Lorem sunt duis officia eiusmod. Aliqua reprehenderit commodo ex non excepteur duis sunt velit enim. Voluptate laboris sint cupidatat ullamco ut ea consectetur et est culpa et culpa duis.";
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
