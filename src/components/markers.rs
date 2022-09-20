#[cfg(feature = "render")]
use {crate::macros::iterable_enum, bevy::ecs::component::Component};

#[cfg(all(feature = "render", not(feature = "globe_view")))]
iterable_enum!(ToolbarButton {
    GenerateWorld,
    SaveWorld,
    LoadWorld,
    Rainfall,
    Temperature,
    PlanetView,
    Contours,
});
#[cfg(all(feature = "render", feature = "globe_view"))]
iterable_enum!(ToolbarButton {
    GenerateWorld,
    SaveWorld,
    LoadWorld,
    Rainfall,
    Temperature,
    PlanetView,
    Contours,
    GlobeView,
});

#[cfg(feature = "render")]
impl From<ToolbarButton> for &'static str {
    fn from(button: ToolbarButton) -> Self {
        match button {
            ToolbarButton::Rainfall => "Toggle rainfall",
            ToolbarButton::Temperature => "Toggle temperature",
            ToolbarButton::Contours => "Toggle contours",
            ToolbarButton::PlanetView => "Cycle view",
            ToolbarButton::GenerateWorld => "Generate new world",
            ToolbarButton::SaveWorld => "Save",
            ToolbarButton::LoadWorld => "Load",
            #[cfg(feature = "globe_view")]
            ToolbarButton::GlobeView => "Toggle globe",
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
