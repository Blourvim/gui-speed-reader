use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ReaderConfig {
    pub words_per_minute: u32,
}

// defaul falues for the config go here
impl ::std::default::Default for ReaderConfig {
    fn default() -> Self {
        Self {
            words_per_minute: 300,
        }
    }
}

