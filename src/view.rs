use iced::{
    widget::{button, Row, Text},
    Alignment, Element,
};

use crate::model::{data::Message, reader::Reader};

impl Reader {
    pub fn view(&self) -> Element<Message> {
        let mut display_buttons = Row::new();

        for (i, rect) in self.display_array.displays.into_iter().enumerate() {
            let mut btn = button(Text::new(rect.content.to_string()).align_y(Alignment::Center).size(20))
                .width(25)
                .height(50);

            if i == (self.display_array.displays.len() / 2) {
                btn = btn.style(button::danger);
            }

            display_buttons = display_buttons.push(btn);
        }
        display_buttons.align_y(Alignment::Center).into()
    }
}
