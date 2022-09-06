#[cfg(feature = "render")]
use std::fmt::Display;

#[cfg(feature = "render")]
#[derive(Default, Debug)]
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
