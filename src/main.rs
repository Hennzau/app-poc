use iced::{self, *};

pub mod panes;

pub fn main() -> iced::Result {
    iced::application("Rine", Rine::update, Rine::view)
        .subscription(Rine::subscription)
        .theme(Rine::theme)
        .font(include_bytes!("../fonts/window-icons.ttf").as_slice())
        .default_font(Rine::FONT)
        .run()
}

struct Rine {
    panes: panes::Panes,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Panes(panes::Message),
}

impl Rine {
    const FONT: Font = Font::MONOSPACE;

    fn new() -> Self {
        Self {
            panes: panes::Panes::new(),
        }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Panes(message) => self.panes.update(message),
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        let panes = self.panes.subscriptions().map(Message::Panes);

        Subscription::batch([panes])
    }

    fn view(&self) -> Element<Message> {
        let panes = self.panes.view().map(Message::Panes);

        widget::container(panes)
            .style(|theme| widget::container::Style {
                background: Some(theme.extended_palette().background.weak.color.into()),
                ..Default::default()
            })
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::Ferra
    }
}

impl Default for Rine {
    fn default() -> Self {
        Self::new()
    }
}
