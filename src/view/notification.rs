//! Notification banner.

use anyhow::Error;
use iced::{button, Alignment, Button, Container, Length, Row, Text};

use crate::app::Message;
use crate::style;
use crate::style::font::IOSEVKA;

#[derive(Debug, PartialEq)]
pub enum Type {
    Error,
}

#[derive(Debug)]
pub struct State {
    button_state: button::State,
    pub type_: Type,
    pub error: Error,
}

impl State {
    pub fn new(type_: Type, error: Error) -> Self {
        Self {
            button_state: button::State::default(),
            type_,
            error,
        }
    }

    pub fn error(error: Error) -> Self {
        Self::new(Type::Error, error)
    }
}

pub fn view(state: &mut State) -> Container<'_, Message> {
    let text = Text::new(state.error.to_string())
        .font(IOSEVKA)
        .width(Length::Fill);

    let close_button = Button::new(&mut state.button_state, Text::new('x').font(IOSEVKA))
        .padding(10)
        .on_press(Message::CloseNotification)
        .style(style::Notification);

    Container::new(
        Row::new()
            .push(text)
            .push(close_button)
            .align_items(Alignment::Center),
    )
    .padding(15)
    .width(Length::Fill)
    .style(style::Notification)
}
