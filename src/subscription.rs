use iced::time;
use std::time::Duration;

use iced::Subscription;

use crate::model::{data::Message, reader::Reader};

impl Reader {
    pub fn subscription(&self) -> Subscription<Message> {
        let message = time::every(Duration::from_secs(1)).map(|_| Message::Increment);
        Subscription::batch(vec![
            message, // , keyboard::on_key_press(handle_hotkey)
        ])
    }
}
