//! Main application logic.

use std::path::PathBuf;

use anyhow::Error;
use iced::keyboard::KeyCode;
use iced::{
    executor, keyboard, Alignment, Application, Column, Command, Container, Element, Length, Row,
    Rule, Subscription, Text,
};
use iced_native::{event, subscription, Event};

use crate::style::color::ORANGE;
use crate::style::font::IOSEVKA_HEAVY_ITALIC;
use crate::view::{notification, select_file};
use crate::{interaction, style};
use crate::{TITLE, VERSION};

#[derive(Debug)]
enum ViewState {
    Initial,
    EditFile,
}

impl Default for ViewState {
    fn default() -> Self {
        Self::Initial
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    SelectFileDialog,
    CloseNotification,
    LoadFile(PathBuf),

    // Keystroke handlers.
    EscPress,
    CmdQPress,
}

#[derive(Debug, Default)]
pub struct TSVConverter {
    view_state: ViewState,
    notification_state: Option<notification::State>,
    select_file_state: select_file::State,
    should_exit: bool,
}

impl Application for TSVConverter {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        format!("{TITLE} {VERSION}")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::SelectFileDialog => {
                self.select_file_state.window_open = true;

                match interaction::select_file() {
                    Ok(opt) => match opt {
                        Some(path) => Command::perform(async move { path }, Message::LoadFile),
                        None => {
                            self.select_file_state.window_open = false;
                            self.notification_state = Some(notification::State::new(
                                notification::Type::SelectFile,
                                Error::msg("Test test test"),
                            ));
                            Command::none()
                        }
                    },
                    Err(err) => {
                        self.select_file_state.window_open = false;
                        self.notification_state = Some(notification::State::new(
                            notification::Type::SelectFile,
                            err,
                        ));
                        Command::none()
                    }
                }
            }
            Message::CloseNotification => {
                self.notification_state = None;
                Command::none()
            }
            Message::LoadFile(path) => {
                self.select_file_state.window_open = false;
                if let Some(state) = &self.notification_state {
                    if state.type_ == notification::Type::SelectFile {
                        self.notification_state = None;
                    }
                }

                Command::none()
            }

            // Keystroke handlers.
            Message::EscPress => {
                if self.notification_state.is_some() {
                    self.notification_state = None;
                }
                Command::none()
            }
            Message::CmdQPress => {
                self.should_exit = true;
                Command::none()
            }
        }
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        subscription::events_with(|event, status| {
            if status == event::Status::Captured {
                return None;
            }

            match event {
                Event::Keyboard(keyboard::Event::KeyPressed { key_code, .. })
                    if key_code == KeyCode::Escape =>
                {
                    Some(Message::EscPress)
                }
                Event::Keyboard(keyboard::Event::KeyPressed {
                    key_code,
                    modifiers,
                }) if modifiers.command() && key_code == KeyCode::Q => Some(Message::CmdQPress),
                _ => None,
            }
        })
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        // Select file button - in the content view in the `Initial` state, or else in the menu.
        let select_file_button = select_file::view(&mut self.select_file_state);

        // Menu bar.
        let menu = {
            let title = Text::new(TITLE.to_uppercase())
                .font(IOSEVKA_HEAVY_ITALIC)
                .color(*ORANGE)
                .size(40);

            let menu_row = Row::new()
                .align_items(Alignment::Center)
                .spacing(20)
                .padding(15)
                .push(title);

            let border_bottom = Rule::horizontal(0).style(style::Menu);

            Container::new(Column::new().push(menu_row).push(border_bottom))
                .width(Length::Fill)
                .style(style::Menu)
        };

        let mut all = Column::new().push(menu);
        if let Some(state) = &mut self.notification_state {
            all = all.push(notification::view(state));
        }
        all = all.push(match self.view_state {
            ViewState::Initial => select_file_button,
            ViewState::EditFile => todo!(),
        });

        Container::new(all)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(style::Main)
            .into()
    }

    fn should_exit(&self) -> bool {
        self.should_exit
    }
}
