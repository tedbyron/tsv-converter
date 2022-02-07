//! View with a single button prompting for a video selection.

use iced::{button, Button, Container, Length, Text};

use crate::app::Message;
use crate::style;
use crate::style::font::IOSEVKA;

#[derive(Debug, Default)]
pub struct State {
    button_state: button::State,
    pub window_open: bool,
}

pub fn view(state: &mut State) -> Container<'_, Message> {
    let mut button = Button::new(
        &mut state.button_state,
        Text::new("Select a video").font(IOSEVKA),
    )
    .padding(10)
    .style(style::Main);

    if !state.window_open {
        button = button.on_press(Message::SelectFileDialog);
    }

    Container::new(button)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
}
