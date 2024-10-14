pub mod controller;
mod model;
mod view;
use std::time::Duration;

use iced::time;
use iced::{widget::button, Element, Subscription, Theme};

struct Reader {
    pub words: Vec<String>,
    pub index: usize,
}

#[derive(Debug, Clone)]
enum Message {
    Increment,
}

impl Default for Reader {
    fn default() -> Self {
        Self::new()
    }
}
impl Reader {
    pub fn new() -> Self {
        let text = "Lorem ipsum dolor sit amet, officia excepteur ex fugiat reprehenderit enim labore culpa sint ad nisi Lorem pariatur mollit ex esse exercitation amet. Nisi anim cupidatat excepteur officia. Reprehenderit nostrud nostrud ipsum Lorem est aliquip amet voluptate voluptate dolor minim nulla est proident. Nostrud officia pariatur ut officia. Sit irure elit esse ea nulla sunt ex occaecat reprehenderit commodo officia dolor Lorem duis laboris cupidatat officia voluptate. Culpa proident adipisicing id nulla nisi laboris ex in Lorem sunt duis officia eiusmod. Aliqua reprehenderit commodo ex non excepteur duis sunt velit enim. Voluptate laboris sint cupidatat ullamco ut ea consectetur et est culpa et culpa duis.";
        let words: Vec<String> = text.split_whitespace().map(String::from).collect();
        println!("setting words");
        Self {
            words: words.clone(),
            index: 0usize,
        }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Increment => {
                println!("incrementing{:?}", &self.index);

                self.index = self.index + 1
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        let word: &str = match self.words.len() {
            0 => "no word",
            _ => &self.words[self.index],
        };

        button(word).into()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        let message = time::every(Duration::from_secs(1)).map(|_| Message::Increment);
        Subscription::batch(vec![
            message, // , keyboard::on_key_press(handle_hotkey)
        ])
    }
    pub fn theme(&self) -> Theme {
        Theme::Dark
    }
}
fn main() -> iced::Result {
    iced::application("title", Reader::update, Reader::view)
        .subscription(Reader::subscription)
        .theme(Reader::theme)
        .run()
}
