use iced::{self, *};

pub fn title_bar(theme: &Theme) -> widget::container::Style {
    let palette = theme.extended_palette();

    widget::container::Style {
        background: None,
        text_color: Some(palette.primary.base.color),
        ..Default::default()
    }
}

pub fn full_pane(theme: &Theme) -> widget::container::Style {
    let palette = theme.extended_palette();

    widget::container::Style {
        background: Some(palette.background.weak.color.into()),
        border: Border {
            width: 2.0,
            color: palette.background.strong.color,
            radius: border::Radius::new(10.0),
        },
        ..Default::default()
    }
}

pub fn full_pane_focus(theme: &Theme) -> widget::container::Style {
    let palette = theme.extended_palette();

    widget::container::Style {
        background: Some(palette.background.weak.color.into()),
        border: Border {
            width: 2.0,
            color: palette.primary.strong.color,
            radius: border::Radius::new(10.0),
        },
        ..Default::default()
    }
}

pub fn pane_content(theme: &Theme) -> widget::container::Style {
    let palette = theme.extended_palette();

    widget::container::Style {
        background: Some(palette.background.weak.color.into()),
        ..Default::default()
    }
}

pub fn icon<'a, Message>(codepoint: char) -> Element<'a, Message> {
    const ICON_FONT: Font = Font::with_name("window-icons");

    iced::widget::text(codepoint)
        .size(12)
        .font(ICON_FONT)
        .into()
}

pub fn button(theme: &Theme, status: widget::button::Status) -> widget::button::Style {
    let palette = theme.extended_palette();
    let mut style = widget::button::primary(theme, status);

    style.background = None;
    style.text_color = match status {
        widget::button::Status::Active => palette.primary.weak.color,
        widget::button::Status::Hovered => palette.primary.strong.color,
        widget::button::Status::Pressed => palette.primary.strong.color,
        widget::button::Status::Disabled => palette.primary.weak.color,
    };

    style
}
