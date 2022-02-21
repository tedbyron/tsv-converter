//! Open repo text button.

use iced::{button, Button, Text};

use crate::app::Message;
use crate::style::font::IOSEVKA;
use crate::{style, REPO};

#[derive(Debug, Default)]
pub struct State {
    button_state: button::State,
}

pub fn view(state: &mut State) -> Button<'_, Message> {
    Button::new(
        &mut state.button_state,
        Text::new(REPO).font(IOSEVKA).size(16),
    )
    .style(style::TextButton)
    .on_press(Message::OpenRepo)
}
