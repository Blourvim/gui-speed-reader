use iced::{
    widget::{button, Row, Text},
    Element,
};

use crate::model::{data::Message, reader::Reader};

impl Reader {
    pub fn view(&self) -> Element<Message> {
        let mut display_buttons = Row::new();

        for rect in self.display_array.displays {
            display_buttons =
                display_buttons.push(button(Text::new(rect.content.to_string())).width(20));
        }
        display_buttons.into()
    }
}
