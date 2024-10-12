use iced::border;
use iced::keyboard;
use iced::widget::{
    button, center, checkbox, column, container, horizontal_rule, horizontal_space, pick_list, row,
    scrollable, text, vertical_rule,
};
use iced::{color, Center, Element, Fill, Font, Shrink, Subscription, Theme};

#[derive(Debug)]
pub struct Layout {
    explain: bool,
    theme: Theme,
}
impl Default for Layout {
    fn default() -> Self {
        Layout {
            explain: false,          // or any other default value
            theme: Theme::CatppuccinMacchiato // Default value for theme
        }
    }
}
#[derive(Debug, Clone)]
pub enum Message {
    ThemeSelector(Theme),
}

impl Layout {

    pub fn update(&mut self, message: Message) {
        match message {
            Message::ThemeSelector(theme) => {
                self.theme = theme;
            }
        }
    }

    pub(crate) fn subscription(&self) -> Subscription<Message> {
        use keyboard::key;
        keyboard::on_key_release(|key, _modifiers| match key {
            _ => None,
        })
    }

    pub(crate) fn view(&self) -> Element<Message> {

        row![]
            .spacing(10)
            .padding(20)
            .into()
    }

    pub(crate) fn theme(&self) -> Theme {
        self.theme.clone()
    }
}

fn column_<'a>() -> Element<'a, Message> {
    column![
        "A column can be used to",
        "lay out widgets vertically.",
        "The amount of space between",
        "elements can be configured!",
    ]
    .spacing(40)
    .into()
}

fn row_<'a>() -> Element<'a, Message> {
    row![button("sdf")]
        // .spacing(40)
        .into()
}

fn space<'a>() -> Element<'a, Message> {
    row!["Left!", horizontal_space(), "Right!"].into()
}

fn application<'a>() -> Element<'a, Message> {
    let header = container(
        row![horizontal_space(), "Header!", horizontal_space(),]
            .padding(10)
            .align_y(Center),
    );

    let sidebar = container(
        column!["Sidebar!"]
            .spacing(40)
            .padding(10)
            .width(200)
            .align_x(Center),
    )
    .style(container::rounded_box)
    .center_y(Fill);

    let content = container(
        scrollable(column!["The end"].spacing(40).align_x(Center).width(Fill)).height(Fill),
    )
    .padding(10);

    column![header, row![sidebar, content]].into()
}
