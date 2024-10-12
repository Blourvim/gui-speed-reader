mod view;
use iced::Theme;
use view::Layout;

fn main() -> iced::Result {
    iced::application(Layout::title, Layout::update, Layout::view)
        .subscription(Layout::subscription)
        .theme(|_| Theme::Nord)
        .run()
}
