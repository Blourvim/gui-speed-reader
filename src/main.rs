use model::reader::Reader;

pub mod controller;
mod model;
pub mod theme;
pub mod view;
pub mod subscription;

fn main() -> iced::Result {
    iced::application("title", Reader::update, Reader::view)
        .subscription(Reader::subscription)
        .theme(Reader::theme)
        .run()
}
