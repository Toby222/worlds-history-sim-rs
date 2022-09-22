use bevy::ecs::component::Component;

#[derive(Component)]
pub(crate) struct Pan2d {
    pub(crate) enabled: bool,
}
impl Pan2d {
    #[must_use]
    pub(crate) const fn new() -> Pan2d {
        Pan2d { enabled: true }
    }
}
