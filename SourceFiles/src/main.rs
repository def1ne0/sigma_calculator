#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod calculator;

use calculator::Calculator;
use iced::{ Size, Theme };

pub fn main() -> iced::Result {
    iced::application(Calculator::default, Calculator::update, Calculator::view)
        .theme(Theme::GruvboxDark)
        .window(iced::window::Settings {
            size: Size::new(400.0, 300.0),
            ..iced::window::Settings::default()
        })
        .run()
}

