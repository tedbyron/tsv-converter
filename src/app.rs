//! Main application logic.

use std::path::PathBuf;

use anyhow::Error;
use iced::keyboard::KeyCode;
use iced::{
    executor, keyboard, Alignment, Application, Column, Command, Container, Element, Length, Row,
    Rule, Subscription, Text,
};
use iced_native::event::Status;
use iced_native::{subscription, Event};

use crate::style::color::{FG, ORANGE};
use crate::style::font::{IOSEVKA, IOSEVKA_HEAVY_ITALIC};
use crate::view::{notification, repo_button, select_file_button};
use crate::{style, util, AUTHORS, REPO, TITLE, VERSION};

#[derive(Debug, PartialEq, Eq)]
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
    OpenRepo,
    SelectFileDialog,
    CloseNotification,
    LoadFile(PathBuf),
    SelectCrop(Crop),

    // Keystroke handlers.
    EscPress,
    CmdQPress,
}

#[derive(Debug, Clone, Copy)]
pub enum Crop {
    Letterbox,
    Zoom,
    Stretch,
}

impl Default for Crop {
    fn default() -> Self {
        Self::Letterbox
    }
}

#[derive(Debug, Default)]
pub struct TSVConverter {
    view_state: ViewState,
    notification: Option<notification::State>,
    repo_button: repo_button::State,
    select_file_button: select_file_button::State,
    file_path: Option<PathBuf>,
    crop: Crop,
    should_exit: bool,
}

impl TSVConverter {
    fn error(&mut self, error: Error) {
        self.notification = Some(notification::State::error(error));
    }
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
            Message::OpenRepo => {
                if let Err(err) = open::that(REPO) {
                    self.error(Error::new(err));
                };
                Command::none()
            }
            Message::SelectFileDialog => {
                self.select_file_button.window_open = true;

                match util::select_file() {
                    Ok(opt) => {
                        if let Some(path) = opt {
                            Command::perform(async move { path }, Message::LoadFile)
                        } else {
                            self.select_file_button.window_open = false;
                            Command::none()
                        }
                    }
                    Err(err) => {
                        self.select_file_button.window_open = false;
                        self.error(err);
                        Command::none()
                    }
                }
            }
            Message::CloseNotification => {
                self.notification = None;
                Command::none()
            }
            Message::LoadFile(path) => {
                self.file_path = Some(path);
                self.select_file_button.window_open = false;
                if self.notification.is_some() {
                    self.notification = None;
                }

                // UNWRAP: `self.file_path` has just been set.
                match util::preview_frame(self.file_path.as_ref().unwrap().as_path(), self.crop) {
                    Ok(_) => (), // TODO
                    Err(err) => self.error(err),
                };

                if self.view_state == ViewState::Initial {
                    self.view_state = ViewState::EditFile;
                }

                Command::none()
            }
            Message::SelectCrop(crop) => {
                self.crop = crop;
                // TODO: Message::SelectCrop handler
                Command::none()
            }

            // Keystroke handlers.
            Message::EscPress => {
                if self.notification.is_some() {
                    self.notification = None;
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
            if status == Status::Captured {
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
                Event::Keyboard(keyboard::Event::KeyPressed {
                    key_code,
                    modifiers,
                }) if modifiers.command() && key_code == KeyCode::O => {
                    Some(Message::SelectFileDialog)
                }
                _ => None,
            }
        })
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        // Select file button - in the content view in the `Initial` state, or else in the menu.
        let select_file_button = select_file_button::view(&mut self.select_file_button);

        // Menu bar.
        let menu = {
            let title = Text::new(TITLE)
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
        if let Some(state) = &mut self.notification {
            all = all.push(notification::view(state));
        }

        // Content view.
        match self.view_state {
            ViewState::Initial => {
                all = all
                    .push(
                        Container::new(select_file_button)
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .center_x()
                            .center_y(),
                    )
                    .push(
                        Column::new()
                            .width(Length::Fill)
                            .spacing(0)
                            .padding(20)
                            .align_items(Alignment::Center)
                            .push(Text::new(AUTHORS).font(IOSEVKA).size(16).color(*FG))
                            .push(repo_button::view(&mut self.repo_button)),
                    );
            }
            ViewState::EditFile => (), // TODO,
        };

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
