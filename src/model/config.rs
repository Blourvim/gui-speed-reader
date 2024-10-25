use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ReaderConfig {
    pub words_per_minute: u32,
    pub debug: bool,
    pub tutorial_text: String,
}

// defaul falues for the config go here
impl ::std::default::Default for ReaderConfig {
    fn default() -> Self {
        Self {
            words_per_minute: 300,
            debug: false,
            tutorial_text:"This is a tutorial text, in order to use this software, first copy text into your clipboard, then run it".to_string(),
        }
    }
}
