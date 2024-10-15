use std::env;

use model::reader::Reader;

pub mod controller;
mod model;
pub mod subscription;
pub mod theme;
pub mod view;

fn main() -> iced::Result {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("usage run <text>")
    }

    iced::application("title", Reader::update, Reader::view)
        .subscription(Reader::subscription)
        .theme(Reader::theme)
        .run()
}
