
use model::reader::Reader;

pub mod controller;
mod model;
pub mod subscription;
pub mod theme;
pub mod view;

fn main() -> iced::Result {
    iced::application("title", Reader::update, Reader::view)
        .subscription(Reader::subscription)
        .theme(Reader::theme)
        .run()
}
