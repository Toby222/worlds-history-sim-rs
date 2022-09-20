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

#[cfg(all(feature = "render", not(feature = "planet_view")))]
toolbar_enum!(
    GenerateWorld,
    SaveWorld,
    LoadWorld,
    Rainfall,
    Temperature,
    Biomes,
    Contours,
);
#[cfg(all(feature = "render", feature = "planet_view"))]
toolbar_enum!(
    GenerateWorld,
    SaveWorld,
    LoadWorld,
    Rainfall,
    Temperature,
    Biomes,
    Contours,
    PlanetView,
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
            #[cfg(feature = "planet_view")]
            ToolbarButton::PlanetView => "Toggle planet view",
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
