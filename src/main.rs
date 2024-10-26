use model::reader::Reader;

pub mod controller;
mod model;
pub mod subscription;
pub mod theme;
pub mod view;

fn main() -> iced::Result {
    iced::application("bloreader", Reader::update, Reader::view)
        .subscription(Reader::subscription)
        .theme(Reader::theme)
        .window(iced::window::Settings {
            max_size: Some(iced::Size::new(800.0, 40.0)),
            position: iced::window::Position::Centered,

            ..Default::default()
        })
        .run()
}
