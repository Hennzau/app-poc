use iced::widget::pane_grid::{self, *};

mod style;
mod titlebar;

mod editor;
mod hub;
mod project;

#[derive(Debug, Clone, Copy)]
pub enum Pane {
    Editor,
    Project,
    Hub,
}

impl Pane {
    pub fn title(&self) -> &str {
        match self {
            Pane::Editor => editor::title(),
            Pane::Project => project::title(),
            Pane::Hub => hub::title(),
        }
    }
}

pub struct Panes {
    pub panes: pane_grid::State<Pane>,
    pub focus: pane_grid::Pane,

    pub editor: editor::Editor,
    pub project: project::Project,
    pub hub: hub::Hub,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Restore,
    Close(pane_grid::Pane),
    Maximize(pane_grid::Pane),
    Clicked(pane_grid::Pane),
    MouseEnter(pane_grid::Pane),
    Dragged(pane_grid::DragEvent),
    Resized(pane_grid::ResizeEvent),

    Editor(editor::Message),
    Project(project::Message),
    Hub(hub::Message),
}

impl Panes {
    pub fn new() -> Self {
        let (mut panes, pane) = pane_grid::State::new(Pane::Project);
        panes.split(Axis::Horizontal, pane, Pane::Hub);
        panes.split(Axis::Vertical, pane, Pane::Editor);

        let focus = pane;
        let editor = editor::Editor::new();
        let project = project::Project::new();
        let hub = hub::Hub::new();

        Self {
            panes,
            focus,
            editor,
            project,
            hub,
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
            Message::Editor(message) => self.editor.update(message),
            Message::Project(message) => self.project.update(message),
            Message::Hub(message) => self.hub.update(message),
            _ => {}
        }
    }

    pub fn subscriptions(&self) -> iced::Subscription<Message> {
        iced::keyboard::on_key_press(|_, _| None)
    }

    pub fn view(&self) -> iced::Element<Message> {
        let pane_grid = PaneGrid::new(&self.panes, |id, pane, is_maximized| {
            let focus = id == self.focus;

            let title_bar =
                titlebar::title_bar(id, pane.clone(), self.panes.len() == 1, is_maximized);

            Content::new({
                let content = match pane {
                    Pane::Editor => self.editor.view().map(Message::Editor),
                    Pane::Project => self.project.view().map(Message::Project),
                    Pane::Hub => self.hub.view().map(Message::Hub),
                };

                iced::widget::mouse_area(
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
                    }),
                )
                .on_enter(Message::MouseEnter(id))
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
