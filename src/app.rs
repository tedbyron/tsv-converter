use std::fs::File;
use std::path::PathBuf;

use iced::alignment::Horizontal;
use iced::{Alignment, Application, Column, Command, Element, Row, Rule, Text};
use iced::{Container, Length};

use crate::font::IOSEVKA_HEAVY_ITALIC;
use crate::style;
use crate::APP_TITLE;
use crate::APP_VERSION;

#[derive(Debug)]
enum State {
    Initial,
    EditFile,
    ProcessFile,
}
impl Default for State {
    fn default() -> Self {
        Self::Initial
    }
}

#[derive(Debug)]
pub enum Message {}

#[derive(Debug, Default)]
pub struct TSVConverter {
    state: State,
    file_path: Option<PathBuf>,
    file: Option<File>,
}

impl Application for TSVConverter {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        format!("{APP_TITLE} {APP_VERSION}")
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let menu = {
            let title = Text::new(APP_TITLE.to_uppercase())
                .font(IOSEVKA_HEAVY_ITALIC)
                .color(*crate::color::ORANGE)
                .size(40)
                .width(Length::Fill)
                .horizontal_alignment(Horizontal::Left);

            let menu_row = Row::new()
                .align_items(Alignment::Center)
                .spacing(20)
                .push(title);

            Container::new(menu_row)
                .width(Length::Fill)
                .padding(15)
                .style(style::Menu)
        };

        let rule = Rule::horizontal(0).style(style::MenuRule);

        let all = Column::new().push(menu).push(rule);

        Container::new(all)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(style::Main)
            .into()
    }
}
