use model::reader::Reader;

pub mod controller;
mod model;
pub mod subscription;
pub mod theme;
pub mod view;

fn main() -> iced::Result {
    iced::application("reader", Reader::update, Reader::view)
        .subscription(Reader::subscription)
        .theme(Reader::theme)
        .window(iced::window::Settings {
            max_size: Some(iced::Size::new(925.0, 3.0)),
            position: iced::window::Position::Centered,
            decorations: false,
            visible: true,
            level: iced::window::Level::AlwaysOnTop,
            ..Default::default()
        })
        .run()
}
