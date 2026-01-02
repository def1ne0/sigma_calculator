use iced::{ Alignment, Background, Element, Fill, Task, Theme };
use iced::widget::{ button, container, row, text, text_input, Button };
use iced::widget::button::Style;
use iced_core::border::Radius;
use iced_core::Vector;
use meval::eval_str;

#[derive(Debug, Clone)]
pub enum Message {
    InputChanged(String),
    AddToInput(String),
    DeleteLastChar,
    ClearInput,
    Evaluate,
    Quit,
}

#[derive(Debug, Clone, Default)]
pub struct Calculator {
    expression: String,
    result: String,
}

fn calc_button_style<'a>() -> impl Fn(&Theme, button::Status) -> Style + 'a {
    |_t, _status|  {
        Style {
            background: Background::Color(iced_core::Color {
                r: 0.31,
                g: 0.29,
                b: 0.27,
                a: 1.0,
            })
                .into(),
            text_color: iced_core::Color {
                r: 0.92,
                g: 0.86,
                b: 0.7,
                a: 1.0,
            },
            border: iced_core::Border {
                color: iced_core::Color::default(),
                width: 5.0,
                radius: Radius {
                    top_right: 10.0,
                    top_left: 10.0,
                    bottom_left: 10.0,
                    bottom_right: 10.0,
                }
            },
            shadow: iced_core::Shadow {
                color: iced_core::Color {
                    r: 0.31,
                    g: 0.29,
                    b: 0.27,
                    a: 0.1,
                },
                blur_radius: 5.0,
                offset: Vector::new(0.5, 0.5),
            },
            ..Style::default()
        }
    }
}

fn calc_button(text: &str, msg: Message) -> Button<'_, Message> {
    button(text)
        .width(50.0)
        .height(50.0)
        .on_press(msg)
        .padding(15)
        .style(calc_button_style())
}

impl Calculator {
    #[allow(unused)]
    pub fn new() -> Calculator {
        Calculator {
            expression: String::new(),
            result: String::new(),
        }
    }

    pub fn update(&mut self, message: Message) -> Task<Message>{
        match message {
            Message::InputChanged(value) => {
                self.expression = value;
                Task::none()
            },
            Message::AddToInput(value) => {
                self.expression += value.trim();
                Task::none()
            },
            Message::DeleteLastChar => {
                self.expression.pop();
                Task::none()
            },
            Message::ClearInput => {
                self.expression.clear();
                Task::none()
            }
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

    pub fn view(&self) -> Element<'_, Message> {
        container (
            iced::widget::column![
                text_input("Input Expression", &self.expression)
                    .on_input(Message::InputChanged),
                text(self.result.clone()),
                row![
                    calc_button("AC", Message::ClearInput),
                    calc_button("C", Message::DeleteLastChar),
                    calc_button("^", Message::AddToInput("^".to_string())),
                    calc_button("/", Message::AddToInput("/".to_string())),
                ]
                    .spacing(10),
                row![
                    calc_button("7", Message::AddToInput("7".to_string())),
                    calc_button("8", Message::AddToInput("8".to_string())),
                    calc_button("9", Message::AddToInput("9".to_string())),
                    calc_button("*", Message::AddToInput("*".to_string())),
                ]
                    .spacing(10),
                row![
                    calc_button("4", Message::AddToInput("4".to_string())),
                    calc_button("5", Message::AddToInput("5".to_string())),
                    calc_button("6", Message::AddToInput("6".to_string())),
                    calc_button("-", Message::AddToInput("-".to_string())),
                ]
                    .spacing(10),
                row![
                    calc_button("1", Message::AddToInput("1".to_string())),
                    calc_button("2", Message::AddToInput("2".to_string())),
                    calc_button("3", Message::AddToInput("3".to_string())),
                    calc_button("+", Message::AddToInput("+".to_string())),
                ]
                    .spacing(10),
                row![
                    calc_button("e", Message::AddToInput(" 2.7182818284".to_string())),
                    calc_button("0", Message::AddToInput("0".to_string())),
                    calc_button(".", Message::AddToInput(".".to_string())),
                    calc_button("=", Message::Evaluate),
                ]
                    .spacing(10),
                button("Quit")
                    .on_press(Message::Quit)
            ]
                .spacing(10)
                .align_x(Alignment::Center)
        )
            .center(Fill)
            .into()
    }
}
