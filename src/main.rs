use iced::{Alignment, Element, Fill};
use iced::widget::{button, column, container, text, text_input};
use meval::eval_str;

pub fn main() -> iced::Result {
    iced::run(Calculator::update, Calculator::view)
}

#[derive(Debug, Clone)]
enum Message {
    InputChanged(String),
    Evaluate,
}

#[derive(Debug, Clone, Default)]
struct Calculator {
    expression: String,
    result: String,
}

impl Calculator {
    #[allow(unused)]
    fn new() -> Calculator {
        Calculator {
            expression: String::new(),
            result: String::new(),
        }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::InputChanged(value) => {
                self.expression = value;
            },
            Message::Evaluate => {
                self.result = eval_str(self.expression.clone())
                    .unwrap()
                    .to_string()
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        container(
            column![
                text_input("Input expression", &self.expression)
                    .on_input(Message::InputChanged),
                button("Evaluate")
                    .on_press(Message::Evaluate),
                text(self.result.trim()),
            ]
                .align_x(Alignment::Center),
        )
            .center(Fill)
            .into()
    }
}

