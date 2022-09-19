#[cfg(feature = "render")]
use {
    crate::{components::markers::ToolbarButton, NORMAL_BUTTON},
    bevy::{
        asset::Handle,
        render::color::Color,
        text::{Font, Text, TextStyle},
        ui::{
            entity::{ButtonBundle, TextBundle},
            widget::Button,
            AlignItems,
            JustifyContent,
            Style,
        },
        utils::default,
    },
};

#[cfg(feature = "render")]
pub(crate) fn toolbar_button() -> ButtonBundle {
    ButtonBundle {
        button: Button,
        style: Style {
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        color: NORMAL_BUTTON.into(),
        ..default()
    }
}

#[cfg(feature = "render")]
pub(crate) fn toolbar_button_text(font: Handle<Font>, which: ToolbarButton) -> TextBundle {
    TextBundle {
        text: Text::from_section(
            which,
            TextStyle {
                font,
                font_size: 20.0,
                color: Color::WHITE,
            },
        ),
        ..default()
    }
}
