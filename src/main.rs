use iced::Element;
use iced::widget::{button, text};

#[derive(Debug, Clone)]
enum Message {
    Increment,
}
fn view(counter: &u64) -> Element<'_, Message> {
    button(text(counter))
        .on_press(Message::Increment)
        .into()
}

fn update(counter: &mut u64, message: Message) {
    match message {
        Message::Increment => *counter += 1,
    }
}

pub fn main() -> iced::Result {
    iced::run(update, view)
}