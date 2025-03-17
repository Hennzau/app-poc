use iced::widget::{horizontal_space, pane_grid::*};
use iced::{self, *};

pub fn title_bar<'a>(
    id: Pane,
    pane: super::Pane,
    only_one: bool,
    maximized: bool,
) -> TitleBar<'a, super::Message> {
    TitleBar::new(
        iced::widget::container(iced::widget::text(format!("#{}", pane.title())).size(12)).padding(
            Padding {
                top: 5.0,
                left: 10.0,
                ..Default::default()
            },
        ),
    )
    .controls(Controls::new({
        let close = iced::widget::button(
            iced::widget::container(super::style::icon('\u{0042}'))
                .padding(Padding::default().left(-4.0)),
        )
        .width(24)
        .height(24)
        .style(super::style::button)
        .on_press(super::Message::Close(id))
        .into();

        let maximize = iced::widget::button(
            iced::widget::container(super::style::icon('\u{0041}'))
                .padding(Padding::default().left(-4.0)),
        )
        .width(24)
        .height(24)
        .style(super::style::button)
        .on_press(super::Message::Maximize(id))
        .into();

        let restore = iced::widget::button(
            iced::widget::container(super::style::icon('\u{0043}'))
                .padding(Padding::default().left(-4.0)),
        )
        .width(24)
        .height(24)
        .style(super::style::button)
        .on_press(super::Message::Restore)
        .into();

        if only_one {
            iced::widget::row([horizontal_space().width(5).into()])
        } else if maximized {
            iced::widget::row([restore, close, horizontal_space().width(5).into()])
        } else {
            iced::widget::row([maximize, close, horizontal_space().width(5).into()])
        }
    }))
    .style(super::style::title_bar)
}
