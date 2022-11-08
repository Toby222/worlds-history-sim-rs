use {
    super::windows,
    crate::resources::OpenedWindows,
    bevy::{
        ecs::{
            change_detection::Mut,
            system::{SystemParam, SystemState},
            world::World,
        },
        log::debug,
        utils::HashMap,
    },
    bevy_egui::egui::{Context, Ui, Window},
    fxhash::FxHasher32,
    std::hash::Hasher,
};

pub(crate) trait WindowSystem: SystemParam {
    fn draw_contents(world: &mut World, state: &mut SystemState<Self>, ui: &mut Ui);
    fn name() -> &'static str;
}

pub(crate) fn render_windows(world: &mut World, ctx: &Context) {
    // TODO: Windows are hard-coded here instead of being iterable, and allows
    // creating new windows that are never rendered.
    // Is that good enough?
    window::<windows::TileInfo>(world, ctx);
    window::<windows::WorldViewSelection>(world, ctx);
    window::<windows::WorldOverlaySelection>(world, ctx);
    window::<windows::SaveLoad>(world, ctx);
}

pub(crate) fn open_window<S: 'static + WindowSystem>(windows: &mut OpenedWindows) {
    windows.open(S::name().into());
}
pub(crate) fn close_window<S: 'static + WindowSystem>(windows: &mut OpenedWindows) {
    windows.close(&S::name().into());
}

fn window<S: 'static + WindowSystem>(world: &mut World, ctx: &Context) {
    // We need to cache `SystemState` to allow for a system's locally tracked state
    if !world.contains_resource::<StateInstances<S>>() {
        // Note, this message should only appear once! If you see it twice in the logs,
        // the function may have been called recursively, and will panic.
        debug!("Init system state {}", std::any::type_name::<S>());
        world.insert_resource(StateInstances::<S> {
            instances: HashMap::new(),
        });
    }
    world.resource_scope(|world, mut states: Mut<'_, StateInstances<S>>| {
        let id: WindowId = S::name().into();
        if !states.instances.contains_key(&id) {
            debug!(
                "Registering system state for window {id:?} of type {}",
                std::any::type_name::<S>()
            );
            _ = states.instances.insert(id, SystemState::new(world));
        }
        // Instead of passing this to open, don't render manually.
        // Saves fetching states, but might fuck up states?
        // TODO: Check that
        if world.resource::<OpenedWindows>().is_open(&id) {
            let cached_state = states.instances.get_mut(&id).unwrap();

            let mut still_open = true;
            Window::new(S::name())
                .resizable(false)
                .open(&mut still_open)
                .title_bar(true)
                .show(ctx, |ui| {
                    S::draw_contents(world, cached_state, ui);
                });
            if !still_open {
                close_window::<S>(&mut world.resource_mut::<OpenedWindows>());
            }

            cached_state.apply(world);
        }
    });
}

#[derive(Default)]
struct StateInstances<T: WindowSystem> {
    instances: HashMap<WindowId, SystemState<T>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct WindowId(pub(crate) u64);
impl WindowId {
    #[must_use]
    pub(crate) fn new(name: &str) -> Self {
        let bytes = name.as_bytes();
        let mut hasher = FxHasher32::default();
        hasher.write(bytes);
        WindowId(hasher.finish())
    }

    // #[must_use]
    // pub(crate) fn with(&self, name: &str) -> Self {
    //     Self::new(&format!("{}{name}", self.0))
    // }
}
impl From<&str> for WindowId {
    #[must_use]
    fn from(str: &str) -> Self {
        Self::new(str)
    }
}
