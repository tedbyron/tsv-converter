//! Select a video button.

use iced::{button, Button, Container, Text};

use crate::app::Message;
use crate::style;
use crate::style::font::IOSEVKA_BOLD;

#[derive(Debug, Default)]
pub struct State {
    button_state: button::State,
    pub window_open: bool,
}

pub fn view(state: &mut State) -> Button<'_, Message> {
    let mut button = Button::new(
        &mut state.button_state,
        Container::new(Text::new("Select a video").font(IOSEVKA_BOLD)).padding([0, 5]),
    )
    .padding(10)
    .style(style::Main);

    if !state.window_open {
        button = button.on_press(Message::SelectFileDialog);
    }

    button
}
