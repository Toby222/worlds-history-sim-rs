#[cfg(feature = "render")]
use bevy::ecs::component::Component;

#[cfg(feature = "render")]
#[derive(Component)]
pub(crate) enum ToolbarButton {
    Rainfall,
    Temperature,
    Contours,
}

#[cfg(feature = "render")]
#[derive(Component)]
pub(crate) struct InfoPanel;
