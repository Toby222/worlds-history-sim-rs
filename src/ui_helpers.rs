#[cfg(feature = "render")]
use bevy::{
    asset::AssetServer,
    ecs::system::Res,
    render::color::Color,
    ui::{
        entity::{ButtonBundle, TextBundle},
        widget::Button,
        AlignItems, JustifyContent, Style,
    },
    utils::default,
};

#[cfg(feature = "render")]
use crate::{components::markers::ToolbarButton, NORMAL_BUTTON};

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
pub(crate) fn toolbar_button_text(
    asset_server: &Res<'_, AssetServer>,
    which: ToolbarButton,
) -> TextBundle {
    TextBundle {
        text: bevy::text::Text::from_section(
            match which {
                ToolbarButton::Rainfall => "Toggle rainfall",
                ToolbarButton::Temperature => "Toggle temperature",
                ToolbarButton::Contours => "Toggle contours",
            },
            bevy::text::TextStyle {
                font: asset_server.load("JuliaMono.ttf"),
                font_size: 20.0,
                color: Color::WHITE,
            },
        ),
        ..default()
    }
}
