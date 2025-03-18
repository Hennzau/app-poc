use iced::widget::text_editor::{self};

pub struct Hub {
    pub content: text_editor::Content,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Message {}

impl Hub {
    pub fn new() -> Self {
        Self {
            content: text_editor::Content::new(),
        }
    }

    pub fn update(&mut self, message: Message) {
        match message {}
    }

    pub fn view(&self) -> iced::Element<Message> {
        iced::widget::text("Welcome to the hub!").into()
    }
}

pub fn title() -> &'static str {
    "Hub"
}
