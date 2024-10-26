use iced::{
    widget::{button, Row, Text},
    Element,
};

use crate::model::{data::Message, reader::Reader};

impl Reader {
    pub fn view(&self) -> Element<Message> {
        let mut display_buttons = Row::new();

        for (i, rect) in self.display_array.displays.into_iter().enumerate() {
            let mut btn = button(Text::new(rect.content.to_string()))
                .width(25)
                .height(40);

            if i == (self.display_array.displays.len() / 2) {
                btn = btn.style(button::danger);
            }

            display_buttons = display_buttons.push(btn);
        }
        display_buttons.into()
    }
}
