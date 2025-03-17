use iced::widget::pane_grid::{self, *};

mod style;
mod titlebar;

mod editor;

#[derive(Debug, Clone, Copy)]
pub enum Pane {
    Editor,
}

impl Pane {
    pub fn title(&self) -> &str {
        match self {
            Pane::Editor => editor::title(),
        }
    }
}

pub struct Panes {
    pub panes: pane_grid::State<Pane>,
    pub focus: pane_grid::Pane,

    pub editor: editor::Editor,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Restore,
    Close(pane_grid::Pane),
    CloseFocused,
    Maximize(pane_grid::Pane),
    Clicked(pane_grid::Pane),
    MouseEnter(pane_grid::Pane),
    Dragged(pane_grid::DragEvent),
    Resized(pane_grid::ResizeEvent),
    Open((pane_grid::Pane, Pane)),

    FocusAdjacent(pane_grid::Direction),
    SwapAdjacent(pane_grid::Direction),

    Editor(editor::Message),
}

impl Panes {
    pub fn new() -> Self {
        let (mut panes, pane) = pane_grid::State::new(Pane::Editor);
        panes.split(Axis::Horizontal, pane, Pane::Editor);
        panes.split(Axis::Vertical, pane, Pane::Editor);

        let focus = pane;
        let editor = editor::Editor::new();

        Self {
            panes,
            focus,
            editor,
        }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Restore => {
                self.panes.restore();
            }
            Message::Close(pane) => {
                self.panes.close(pane);
            }
            Message::CloseFocused => {
                self.panes.close(self.focus);
            }
            Message::Maximize(pane) => {
                self.panes.maximize(pane);
            }
            Message::Clicked(pane) => {
                self.focus = pane;
            }
            Message::MouseEnter(pane) => {
                self.focus = pane;
            }
            Message::Dragged(pane_grid::DragEvent::Dropped { pane, target }) => {
                self.panes.drop(pane, target);
            }
            Message::Resized(pane_grid::ResizeEvent { split, ratio }) => {
                self.panes.resize(split, ratio);
            }
            Message::FocusAdjacent(direction) => {
                self.focus = match self.panes.adjacent(self.focus, direction) {
                    Some(pane) => pane,
                    None => self.focus,
                };
            }
            Message::SwapAdjacent(direction) => {
                let adjacent = self.panes.adjacent(self.focus, direction);
                if let Some(adjacent) = adjacent {
                    self.panes.swap(self.focus, adjacent);
                } else {
                    // Quick fix, in 0.13.1 move_to_edge close and open a new pane with another label. It's fixed in main but
                    // here is a quick fix. First we get all ids, (first component of the tuple)
                    let all_panes = self
                        .panes
                        .iter()
                        .map(|(id, _)| id.clone())
                        .collect::<Vec<_>>();

                    match direction {
                        Direction::Up => self.panes.move_to_edge(self.focus, Edge::Top),
                        Direction::Down => self.panes.move_to_edge(self.focus, Edge::Bottom),
                        Direction::Left => self.panes.move_to_edge(self.focus, Edge::Left),
                        Direction::Right => self.panes.move_to_edge(self.focus, Edge::Right),
                    }

                    let new_panes = self
                        .panes
                        .iter()
                        .map(|(id, _)| id.clone())
                        .collect::<Vec<_>>();

                    // Find the new one
                    match new_panes.iter().find(|&id| !all_panes.contains(id)) {
                        Some(id) => self.focus = *id,
                        None => {}
                    }
                }
            }
            Message::Editor(message) => self.editor.update(message),
            _ => {}
        }
    }

    pub fn subscriptions(&self) -> iced::Subscription<Message> {
        iced::keyboard::on_key_press(|key, modifiers| {
            let command = modifiers.command() && !modifiers.shift();
            let command_and_shift = modifiers.command() && modifiers.shift();

            if command {
                match key.as_ref() {
                    iced::keyboard::Key::Named(key) => match key {
                        iced::keyboard::key::Named::ArrowUp => {
                            Some(Message::FocusAdjacent(Direction::Up))
                        }
                        iced::keyboard::key::Named::ArrowDown => {
                            Some(Message::FocusAdjacent(Direction::Down))
                        }
                        iced::keyboard::key::Named::ArrowLeft => {
                            Some(Message::FocusAdjacent(Direction::Left))
                        }
                        iced::keyboard::key::Named::ArrowRight => {
                            Some(Message::FocusAdjacent(Direction::Right))
                        }
                        _ => None,
                    },
                    iced::keyboard::Key::Character("w") => Some(Message::CloseFocused),
                    _ => None,
                }
            } else if command_and_shift {
                match key.as_ref() {
                    iced::keyboard::Key::Named(key) => match key {
                        iced::keyboard::key::Named::ArrowUp => {
                            Some(Message::SwapAdjacent(Direction::Up))
                        }
                        iced::keyboard::key::Named::ArrowDown => {
                            Some(Message::SwapAdjacent(Direction::Down))
                        }
                        iced::keyboard::key::Named::ArrowLeft => {
                            Some(Message::SwapAdjacent(Direction::Left))
                        }
                        iced::keyboard::key::Named::ArrowRight => {
                            Some(Message::SwapAdjacent(Direction::Right))
                        }
                        _ => None,
                    },
                    _ => None,
                }
            } else {
                None
            }
        })
    }

    pub fn view(&self) -> iced::Element<Message> {
        let pane_grid = PaneGrid::new(&self.panes, |id, pane, is_maximized| {
            let focus = id == self.focus;

            let title_bar =
                titlebar::title_bar(id, pane.clone(), self.panes.len() == 1, is_maximized);

            Content::new({
                let content = match pane {
                    Pane::Editor => self.editor.view().map(Message::Editor),
                };

                iced::widget::container(
                    iced::widget::container(iced::widget::container(content).padding(5))
                        .style(style::pane_content)
                        .width(iced::Length::Fill)
                        .height(iced::Length::Fill),
                )
                .padding(iced::Padding {
                    top: -1.0,
                    right: 5.0,
                    bottom: 5.0,
                    left: 5.0,
                })
            })
            .title_bar(title_bar)
            .style(if focus {
                style::full_pane_focus
            } else {
                style::full_pane
            })
        })
        .width(iced::Length::Fill)
        .height(iced::Length::Fill)
        .spacing(5)
        .on_click(Message::Clicked)
        .on_drag(Message::Dragged)
        .on_resize(10, Message::Resized);

        iced::widget::container(pane_grid)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .padding(5)
            .into()
    }
}
