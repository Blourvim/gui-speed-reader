mod view;
use iced::Theme;
use view::Layout;

fn main() -> iced::Result {
    iced::application("title", Layout::update, Layout::view)
        .subscription(Layout::subscription)
        .theme(Layout::theme)
        .run()
}
