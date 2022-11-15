#[cfg(feature = "render")]
use {crate::gui::WindowId, bevy::utils::HashSet, std::fmt::Display};
use {
    bevy::{prelude::Resource, tasks::Task},
    crossbeam_channel::{bounded, Receiver, Sender},
    planet::{World, WorldGenError},
};

#[cfg(feature = "render")]
#[derive(Default, Debug, Resource)]
pub struct CursorMapPosition {
    pub x: i32,
    pub y: i32,
}
#[cfg(feature = "render")]
impl Display for CursorMapPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_fmt(format_args!("x: {}, y: {}", self.x, self.y))
    }
}

#[cfg(feature = "render")]
#[derive(Resource, Default)]
pub struct ShouldRedraw(pub bool);
#[cfg(feature = "render")]
#[derive(Default, Resource)]
pub struct OpenedWindows(HashSet<WindowId>);

#[cfg(feature = "render")]
impl OpenedWindows {
    pub fn open(&mut self, id: WindowId) {
        // Ignore opening already opened windows
        _ = self.0.insert(id);
    }

    pub fn close(&mut self, id: &WindowId) {
        // Ignore closing already closed windows
        _ = self.0.remove(id);
    }

    pub fn is_open(&self, id: &WindowId) -> bool {
        self.0.contains(id)
    }
}

#[derive(Resource)]
pub struct GenerateWorldProgressChannel(Sender<(f32, String)>, Receiver<(f32, String)>);

impl GenerateWorldProgressChannel {
    pub fn new() -> Self {
        bounded(1).into()
    }

    pub fn sender(&self) -> Sender<(f32, String)> {
        self.0.clone()
    }

    pub fn receiver(&self) -> &Receiver<(f32, String)> {
        &self.1
    }
}
impl Default for GenerateWorldProgressChannel {
    fn default() -> Self {
        Self::new()
    }
}
impl From<(Sender<(f32, String)>, Receiver<(f32, String)>)> for GenerateWorldProgressChannel {
    fn from(value: (Sender<(f32, String)>, Receiver<(f32, String)>)) -> Self {
        Self(value.0, value.1)
    }
}
#[derive(Default, Resource)]
pub struct GenerateWorldTask(pub Option<Task<Result<World, WorldGenError>>>);
