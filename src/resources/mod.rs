use {crate::gui::WindowId, bevy::utils::HashSet, std::fmt::Display};

#[derive(Default, Debug)]
pub(crate) struct CursorMapPosition {
    pub(crate) x: i32,
    pub(crate) y: i32,
}
impl Display for CursorMapPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("x: {}, y: {}", self.x, self.y))
    }
}

pub(crate) struct ShouldRedraw(pub(crate) bool);
impl Default for ShouldRedraw {
    fn default() -> Self {
        Self(true)
    }
}

#[derive(Default)]
pub(crate) struct OpenedWindows(HashSet<WindowId>);

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
