//! This has all the logic regarding the cliboard history
use arboard::ImageData;
use iced::{
    Length::Fill,
    Theme,
    alignment::Vertical,
    widget::{Button, Row, Text, container},
};

use crate::{app::Message, commands::Function};

/// The kinds of clipboard content that rustcast can handle and their contents
#[derive(Debug, Clone)]
pub enum ClipBoardContentType {
    Text(String),
    Image(ImageData<'static>),
}

impl ClipBoardContentType {
    /// Returns the iced element for rendering the clipboard item
    pub fn render_clipboard_item(&self) -> impl Into<iced::Element<'_, Message>> {
        let mut tile = Row::new().width(Fill).height(55);

        let text = match self {
            ClipBoardContentType::Text(text) => text,
            ClipBoardContentType::Image(_) => "<img>",
        };

        tile = tile.push(
            Button::new(
                Text::new(text.to_owned())
                    .height(Fill)
                    .width(Fill)
                    .align_y(Vertical::Center),
            )
            .on_press(Message::RunFunction(Function::CopyToClipboard(
                self.to_owned(),
            )))
            .style(|_, _| iced::widget::button::Style {
                background: Some(iced::Background::Color(
                    Theme::KanagawaDragon.palette().background,
                )),
                text_color: Theme::KanagawaDragon.palette().text,
                ..Default::default()
            })
            .width(Fill)
            .height(55),
        );

        container(tile)
            .style(|_| iced::widget::container::Style {
                text_color: Some(Theme::KanagawaDragon.palette().text),
                background: Some(iced::Background::Color(
                    Theme::KanagawaDragon.palette().background,
                )),
                ..Default::default()
            })
            .width(Fill)
            .height(Fill)
    }
}

impl PartialEq for ClipBoardContentType {
    /// Let cliboard items be comparable
    fn eq(&self, other: &Self) -> bool {
        if let Self::Text(a) = self
            && let Self::Text(b) = other
        {
            return a == b;
        } else if let Self::Image(image_data) = self
            && let Self::Image(other_image_data) = other
        {
            return image_data.bytes == other_image_data.bytes;
        }
        false
    }
}
