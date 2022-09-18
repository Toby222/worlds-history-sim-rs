#[cfg(feature = "render")]
use bevy::ecs::component::Component;

#[cfg(feature = "render")]
macro_rules! toolbar_enum {
    ($($Variant:ident),*$(,)?) =>
    {
        #[derive(Debug, Component, Copy, Clone)]
        pub enum ToolbarButton {
            $($Variant),*,
        }
        impl ToolbarButton {
            pub const BUTTONS: &'static [ToolbarButton] = &[$(ToolbarButton::$Variant),*];
        }
    }
}

#[cfg(feature = "render")]
toolbar_enum!(
    GenerateWorld,
    SaveWorld,
    LoadWorld,
    Rainfall,
    Temperature,
    Biomes,
    Contours,
);

#[cfg(feature = "render")]
impl From<ToolbarButton> for &'static str {
    fn from(button: ToolbarButton) -> Self {
        match button {
            ToolbarButton::Rainfall => "Toggle rainfall",
            ToolbarButton::Temperature => "Toggle temperature",
            ToolbarButton::Contours => "Toggle contours",
            ToolbarButton::Biomes => "Toggle biomes",
            ToolbarButton::GenerateWorld => "Generate new world",
            ToolbarButton::SaveWorld => "Save",
            ToolbarButton::LoadWorld => "Load",
        }
    }
}
#[cfg(feature = "render")]
impl From<&ToolbarButton> for &'static str {
    fn from(button: &ToolbarButton) -> Self {
        (*button).into()
    }
}

#[cfg(feature = "render")]
impl From<ToolbarButton> for String {
    fn from(button: ToolbarButton) -> Self {
        <&'static str>::from(button).into()
    }
}

#[cfg(feature = "render")]
impl From<&ToolbarButton> for String {
    fn from(button: &ToolbarButton) -> Self {
        <&'static str>::from(button).into()
    }
}

#[cfg(feature = "render")]
#[derive(Component)]
pub(crate) struct InfoPanel;
