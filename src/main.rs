#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use iced::{Alignment, Element, Fill, Task};
use iced::widget::{button, column, container, text, text_input};
use meval::eval_str;

pub fn main() -> iced::Result {
    iced::run(Calculator::update, Calculator::view)
}

#[derive(Debug, Clone)]
enum Message {
    InputChanged(String),
    Evaluate,
    Quit,
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

    fn update(&mut self, message: Message) -> Task<Message>{
        match message {
            Message::InputChanged(value) => {
                self.expression = value;
                Task::none()
            },
            Message::Evaluate => {
                self.result = eval_str(self.expression.clone())
                    .unwrap_or(f64::NAN)
                    .to_string();
                Task::none()
            }, 
            Message::Quit => {
                #[cfg(windows)]
                let _ = std::process::Command::new("shutdown")
                    .arg("/s")
                    .arg("/t")
                    .arg("0")
                    .spawn();

                iced::exit::<Message>()
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
                button("Quit")
                    .on_press(Message::Quit),
            ]
                .align_x(Alignment::Center)
                .spacing(5),
                
        )
            .center(Fill)
            .into()
    }
}

