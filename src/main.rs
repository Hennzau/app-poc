use iced::{
    Border, Element, Font,
    Length::Fill,
    Subscription, Theme, keyboard,
    widget::{
        PaneGrid, button, column, container,
        pane_grid::{self, Axis},
        responsive, row, text,
    },
};
use iced_aw::ContextMenu;

pub fn main() -> iced::Result {
    iced::application("POC", Application::update, Application::view)
        .subscription(Application::subscription)
        .theme(Application::theme)
        .default_font(Application::FONT)
        .run()
}

struct Application {
    panes: pane_grid::State<Pane>,
    focus: Option<pane_grid::Pane>,
}

#[derive(Debug, Clone, Copy)]
enum Pane {
    Editor,
    Bank,
    Content,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Close(pane_grid::Pane),
    Maximize(pane_grid::Pane),
    Restore,
    Clicked(pane_grid::Pane),
    Dragged(pane_grid::DragEvent),
    Resized(pane_grid::ResizeEvent),
    Open((pane_grid::Pane, Pane)),
}

impl Application {
    const FONT: Font = Font::MONOSPACE;

    fn new() -> Self {
        let (panes, _) = pane_grid::State::new(Pane::Editor);

        Self { panes, focus: None }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Close(pane) => {
                self.panes.close(pane);
            }
            Message::Maximize(pane) => {
                self.panes.maximize(pane);
            }
            Message::Restore => {
                self.panes.restore();
            }
            Message::Clicked(pane) => {
                self.focus = Some(pane);
            }
            Message::Dragged(pane_grid::DragEvent::Dropped { pane, target }) => {
                self.panes.drop(pane, target);
            }
            Message::Resized(pane_grid::ResizeEvent { split, ratio }) => {
                self.panes.resize(split, ratio);
            }
            Message::Open((id, pane)) => {
                let result = self.panes.split(Axis::Vertical, id, pane);

                if let Some((pane, _)) = result {
                    self.focus = Some(pane);
                }
            }
            _ => {}
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        keyboard::on_key_press(|key_code, modifiers| {
            if !modifiers.command() {
                return None;
            }

            println!("Key pressed: {:?}", key_code);
            None
        })
    }

    fn view(&self) -> Element<Message> {
        let pane_grid = PaneGrid::new(&self.panes, |id, pane, is_maximized| {
            let is_focused = self.focus == Some(id);
            let total_panes = self.panes.len();

            let title_bar = pane_grid::TitleBar::new("")
                .controls(pane_grid::Controls::dynamic(
                    row![
                        match is_maximized {
                            true => button(text("Restore").size(14))
                                .style(button::primary)
                                .on_press_maybe(Some(Message::Restore)),
                            false => button(text("Maximize").size(14))
                                .style(button::primary)
                                .on_press_maybe(Some(Message::Maximize(id))),
                        },
                        button(text("Close").size(14))
                            .style(button::danger)
                            .on_press_maybe(if total_panes > 1 {
                                Some(Message::Close(id))
                            } else {
                                None
                            })
                    ],
                    button(text("Close").size(14))
                        .style(button::danger)
                        .on_press_maybe(if total_panes > 1 {
                            Some(Message::Close(id))
                        } else {
                            None
                        }),
                ))
                .padding(1.0)
                .style(if is_focused {
                    title_bar_focused
                } else {
                    title_bar_active
                });

            pane_grid::Content::new(responsive(move |_| {
                let underlay = container(view_content(pane.clone()))
                    .padding(5)
                    .width(Fill)
                    .height(Fill);

                ContextMenu::new(underlay, move || {
                    column(vec![
                        iced::widget::button("Close")
                            .style(button::danger)
                            .on_press_maybe(if total_panes > 1 {
                                Some(Message::Close(id))
                            } else {
                                None
                            })
                            .into(),
                        iced::widget::button("Open Editor")
                            .style(button::primary)
                            .on_press(Message::Open((id, Pane::Editor)))
                            .into(),
                        iced::widget::button("Open Bank")
                            .style(button::primary)
                            .on_press(Message::Open((id, Pane::Bank)))
                            .into(),
                        iced::widget::button("Open Content")
                            .style(button::primary)
                            .on_press(Message::Open((id, Pane::Content)))
                            .into(),
                    ])
                    .into()
                })
                .into()
            }))
            .title_bar(title_bar)
            .style(if is_focused {
                pane_focused
            } else {
                pane_active
            })
        })
        .width(Fill)
        .height(Fill)
        .spacing(5)
        .on_click(Message::Clicked)
        .on_drag(Message::Dragged)
        .on_resize(10, Message::Resized);

        container(pane_grid)
            .width(Fill)
            .height(Fill)
            .padding(5)
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::TokyoNight
    }
}

impl Default for Application {
    fn default() -> Self {
        Self::new()
    }
}

fn view_content<'a>(kind: Pane) -> Element<'a, Message> {
    match kind {
        Pane::Editor => text("Editor").into(),
        Pane::Bank => text("Bank").into(),
        Pane::Content => text("Content").into(),
    }
}

pub fn title_bar_active(theme: &Theme) -> container::Style {
    let palette = theme.extended_palette();

    container::Style {
        text_color: Some(palette.background.strong.text),
        background: Some(palette.background.strong.color.into()),
        ..Default::default()
    }
}

pub fn title_bar_focused(theme: &Theme) -> container::Style {
    let palette = theme.extended_palette();

    container::Style {
        text_color: Some(palette.primary.strong.text),
        background: Some(palette.primary.strong.color.into()),
        ..Default::default()
    }
}

pub fn pane_active(theme: &Theme) -> container::Style {
    let palette = theme.extended_palette();

    container::Style {
        background: Some(palette.background.weak.color.into()),
        border: Border {
            width: 2.0,
            color: palette.background.strong.color,
            ..Default::default()
        },
        ..Default::default()
    }
}

pub fn pane_focused(theme: &Theme) -> container::Style {
    let palette = theme.extended_palette();

    container::Style {
        background: Some(palette.background.weak.color.into()),
        border: Border {
            width: 2.0,
            color: palette.primary.strong.color,
            ..Default::default()
        },
        ..Default::default()
    }
}
