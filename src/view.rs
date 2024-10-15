use iced::{
    mouse::Button,
    theme,
    widget::{button, Row, Text},
    Color, Element, Theme,
};

use crate::model::{data::Message, reader::Reader};

fn button_style() -> iced::widget::button::Style {
    button::Style {
        text_color: Color {
            r: 100.0,
            ..Default::default()
        },
        ..Default::default()
    }
}
impl Reader {
    pub fn view(&self) -> Element<Message> {
        let mut display_buttons = Row::new();

        for (i, rect) in self.display_array.displays.into_iter().enumerate() {
            let mut btn = button(Text::new(rect.content.to_string())).width(25);

            if i == (self.display_array.displays.len() / 2) {
                btn = btn.style(button::danger);
            }

            display_buttons = display_buttons.push(btn);
        }
        display_buttons.into()
    }
}
