#[cfg(feature = "render")]
use {crate::gui::WindowId, bevy::utils::HashSet, std::fmt::Display};
use {
    bevy::{prelude::Resource, tasks::Task},
    planet::{World, WorldGenError},
};

#[cfg(feature = "render")]
#[derive(Default, Debug, Resource)]
pub(crate) struct CursorMapPosition {
    pub(crate) x: i32,
    pub(crate) y: i32,
}
#[cfg(feature = "render")]
impl Display for CursorMapPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("x: {}, y: {}", self.x, self.y))
    }
}

#[cfg(feature = "render")]
#[derive(Resource, Default)]
pub(crate) struct ShouldRedraw(pub(crate) bool);
#[cfg(feature = "render")]
#[derive(Default, Resource)]
pub(crate) struct OpenedWindows(HashSet<WindowId>);

#[cfg(feature = "render")]
impl OpenedWindows {
    pub(crate) fn open(&mut self, id: WindowId) {
        // Ignore opening already opened windows
        _ = self.0.insert(id);
    }

    pub(crate) fn close(&mut self, id: &WindowId) {
        // Ignore closing already closed windows
        _ = self.0.remove(id);
    }

    pub(crate) fn is_open(&self, id: &WindowId) -> bool {
        self.0.contains(id)
    }
}

#[derive(Default, Resource)]
pub struct GenerateWorldTask(pub Option<Task<Result<World, WorldGenError>>>);
