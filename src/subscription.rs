use iced::time;
use std::time::Duration;

use iced::Subscription;

use crate::model::{data::Message, reader::Reader};

impl Reader {
    pub fn subscription(&self) -> Subscription<Message> {
        // cycle time = 60*1000 miliseconds/ words per minute
        let cycle_time = 60_000 / self.config.words_per_minute;

        let message =
            time::every(Duration::from_millis(cycle_time.into())).map(|_| Message::Increment);
        Subscription::batch(vec![
            message, // , keyboard::on_key_press(handle_hotkey)
        ])
    }
}
