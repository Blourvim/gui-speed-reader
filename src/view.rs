use iced::border;
use iced::keyboard;
use iced::widget::{
    button, center, checkbox, column, container, horizontal_rule, horizontal_space, pick_list, row,
    scrollable, text, vertical_rule,
};
use iced::{color, Center, Element, Fill, Font, Shrink, Subscription, Theme};


#[derive(Default, Debug)]
pub struct Layout {
    example: Example,
    explain: bool,
    theme: Theme,
}

#[derive(Debug, Clone)]
pub enum Message {
    Next,
    Previous,
    ExplainToggled(bool),
    ThemeSelector(Theme),
}

impl Layout {
    pub fn title(&self) -> String {
        format!("{} - Layout - Iced", self.example.title)
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Next => {
                self.example = self.example.next();
            }
            Message::Previous => {
                self.example = self.example.previous();
            }
            Message::ExplainToggled(explain) => {
                self.explain = explain;
            }
            Message::ThemeSelector(theme) => {
                self.theme = theme;
            }
        }
    }

    pub(crate) fn subscription(&self) -> Subscription<Message> {
        use keyboard::key;
        keyboard::on_key_release(|key, _modifiers| match key {
            keyboard::Key::Named(key::Named::ArrowLeft) => Some(Message::Previous),
            keyboard::Key::Named(key::Named::ArrowRight) => Some(Message::Next),
            _ => None,
        })
    }

    pub(crate) fn view(&self) -> Element<Message> {
        let header = row![
            text(self.example.title).size(20).font(Font::MONOSPACE),
            horizontal_space(),
            checkbox("Explain", self.explain).on_toggle(Message::ExplainToggled),
            pick_list(Theme::ALL, Some(&self.theme), Message::ThemeSelector)
        ]
        .spacing(20)
        .align_y(Center);

        let example = center(if self.explain {
            self.example.view().explain(color!(0x0000ff))
        } else {
            self.example.view()
        })
        .style(|theme| {
            let palette = theme.extended_palette();

            container::Style::default()
                .border(border::color(palette.background.strong.color).width(4))
        })
        .padding(4);

        let controls = row([
            (!self.example.is_first()).then_some(
                button("← Previous")
                    .padding([5, 10])
                    .on_press(Message::Previous)
                    .into(),
            ),
            Some(horizontal_space().into()),
            (!self.example.is_last()).then_some(
                button("Next →")
                    .padding([5, 10])
                    .on_press(Message::Next)
                    .into(),
            ),
        ]
        .into_iter()
        .flatten());

        column![header, example, controls]
            .spacing(10)
            .padding(20)
            .into()
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Example {
    title: &'static str,
    view: fn() -> Element<'static, Message>,
}

impl Example {
    const LIST: &'static [Self] = &[
        Self {
            title: "Centered",
            view: centered,
        },
        Self {
            title: "Column",
            view: column_,
        },
        Self {
            title: "Row",
            view: row_,
        },
        Self {
            title: "Space",
            view: space,
        },
        Self {
            title: "Application",
            view: application,
        },
        Self {
            title: "Quotes",
            view: quotes,
        },
    ];

    fn is_first(self) -> bool {
        Self::LIST.first() == Some(&self)
    }

    fn is_last(self) -> bool {
        Self::LIST.last() == Some(&self)
    }

    fn previous(self) -> Self {
        let Some(index) = Self::LIST.iter().position(|&example| example == self) else {
            return self;
        };

        Self::LIST
            .get(index.saturating_sub(1))
            .copied()
            .unwrap_or(self)
    }

    fn next(self) -> Self {
        let Some(index) = Self::LIST.iter().position(|&example| example == self) else {
            return self;
        };

        Self::LIST.get(index + 1).copied().unwrap_or(self)
    }

    fn view(&self) -> Element<Message> {
        (self.view)()
    }
}

impl Default for Example {
    fn default() -> Self {
        Self::LIST[0]
    }
}

fn centered<'a>() -> Element<'a, Message> {
    center(text("I am centered!").size(50)).into()
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

fn quotes<'a>() -> Element<'a, Message> {
    fn quote<'a>(content: impl Into<Element<'a, Message>>) -> Element<'a, Message> {
        row![vertical_rule(2), content.into()]
            .spacing(10)
            .height(Shrink)
            .into()
    }

    fn reply<'a>(
        original: impl Into<Element<'a, Message>>,
        reply: impl Into<Element<'a, Message>>,
    ) -> Element<'a, Message> {
        column![quote(original), reply.into()].spacing(10).into()
    }

    column![
        reply(
            reply("This is the original message", "This is a reply"),
            "This is another reply",
        ),
        horizontal_rule(1),
        "A separator ↑",
    ]
    .width(Shrink)
    .spacing(10)
    .into()
}
