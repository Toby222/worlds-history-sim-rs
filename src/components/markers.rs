#[cfg(feature = "render")]
use bevy::ecs::component::Component;

#[cfg(feature = "render")]
macro_rules! define_enum {
    ($Name:ident { $($Variant:ident),* $(,)* }) =>
    {
        #[derive(Debug, Component, Copy, Clone)]
        pub enum $Name {
            $($Variant),*,
        }
        impl $Name {
            pub const ITEMS: &'static [$Name] = &[$($Name::$Variant),*];
        }
    }
}

#[cfg(feature = "render")]
define_enum!(ToolbarButton {
    GenerateWorld,
    SaveWorld,
    LoadWorld,
    Rainfall,
    Temperature,
    Contours,
});

#[cfg(feature = "render")]
impl From<ToolbarButton> for &'static str {
    fn from(button: ToolbarButton) -> Self {
        match button {
            ToolbarButton::Rainfall => "Toggle rainfall",
            ToolbarButton::Temperature => "Toggle temperature",
            ToolbarButton::Contours => "Toggle contours",
            ToolbarButton::GenerateWorld => "Generate new world",
            ToolbarButton::SaveWorld => "Save",
            ToolbarButton::LoadWorld => "Load",
        }
    }
}
#[cfg(feature = "render")]
impl From<&ToolbarButton> for &'static str {
    fn from(button: &ToolbarButton) -> Self {
        match button {
            ToolbarButton::Rainfall => "Toggle rainfall",
            ToolbarButton::Temperature => "Toggle temperature",
            ToolbarButton::Contours => "Toggle contours",
            ToolbarButton::GenerateWorld => "Generate new world",
            ToolbarButton::SaveWorld => "Save",
            ToolbarButton::LoadWorld => "Load",
        }
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
