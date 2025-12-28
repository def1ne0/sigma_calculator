use iced::{Element, Fill, Alignment};
use iced::widget::{button, row, column, text, text_input, container};

#[derive(Debug, Clone, Default)]
struct Calc {
    first_value: String,
    second_value: String,
    result_value: String,
}

#[derive(Debug, Clone)]
enum Message {
    Plus,
    Minus,
    GetFirstValue(String),
    GetSecondValue(String),
}

impl Calc {
    fn update(&mut self, message: Message) {
        match message {
            Message::Plus => {
                let f_v = self.first_value.trim().parse::<f64>().unwrap();
                let s_v = self.second_value.trim().parse::<f64>().unwrap();

                self.result_value = (f_v + s_v).to_string();
            },
            Message::Minus => {
                let f_v = self.first_value.trim().parse::<f64>().unwrap();
                let s_v = self.second_value.trim().parse::<f64>().unwrap();

                self.result_value = (f_v - s_v).to_string();
            },
            Message::GetFirstValue(value) => {
                self.first_value = value.clone();
            },
            Message::GetSecondValue(value) => {
                self.second_value = value.clone();
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        container(
            column![
                text_input("Введите первое число", &self.first_value)
                    .on_input(Message::GetFirstValue)
                    .padding(10),
                row![
                    button("+").on_press(Message::Plus),
                    button("-").on_press(Message::Minus),
                ],
                text_input("Введите второе число", &self.second_value)
                    .on_input(Message::GetSecondValue)
                    .padding(10),
                text(self.result_value.trim())
                    .center(),
            ]
                .align_x(Alignment::Center)
        )
            .center(Fill)
            .into()
    }
}

fn main() -> iced::Result {
    iced::run(Calc::update, Calc::view)
}
