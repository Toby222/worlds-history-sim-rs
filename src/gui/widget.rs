use {
    bevy::{
        ecs::{
            change_detection::Mut,
            system::{SystemParam, SystemState},
            world::World,
        },
        log::debug,
        prelude::*,
        utils::HashMap,
    },
    bevy_egui::egui::Ui,
    fxhash::FxHasher32,
    std::hash::Hasher,
};

pub(crate) trait WidgetSystem: SystemParam {
    fn render(world: &mut World, state: &mut SystemState<Self>, ui: &mut Ui, id: WidgetId);
}

pub(crate) fn widget<S: 'static + WidgetSystem>(world: &mut World, ui: &mut Ui, id: WidgetId) {
    // We need to cache `SystemState` to allow for a system's locally tracked state
    if !world.contains_resource::<StateInstances<S>>() {
        // Note, this message should only appear once! If you see it twice in the logs,
        // the function may have been called recursively, and will panic.
        debug!("Init system state {}", std::any::type_name::<S>());
        world.insert_resource(StateInstances::<S> {
            instances: HashMap::new(),
        });
    }
    world.resource_scope(|world, mut states: Mut<StateInstances<S>>| {
        if !states.instances.contains_key(&id) {
            debug!(
                "Registering system state for widget {id:?} of type {}",
                std::any::type_name::<S>()
            );
            _ = states.instances.insert(id, SystemState::new(world));
        }
        let cached_state = states.instances.get_mut(&id).unwrap();
        S::render(world, cached_state, ui, id);
        cached_state.apply(world);
    });
}

/// A UI widget may have multiple instances. We need to ensure the local state
/// of these instances is not shared. This hashmap allows us to dynamically
/// store instance states.
#[derive(Default, Resource)]
struct StateInstances<T: WidgetSystem + 'static> {
    instances: HashMap<WidgetId, SystemState<T>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct WidgetId(pub(crate) u64);
impl WidgetId {
    #[must_use]
    pub(crate) fn new(name: &str) -> Self {
        let bytes = name.as_bytes();
        let mut hasher = FxHasher32::default();
        hasher.write(bytes);
        WidgetId(hasher.finish())
    }

    // #[must_use]
    // pub(crate) fn with(&self, name: &str) -> Self {
    //     Self::new(&format!("{}{name}", self.0))
    // }
}
impl From<&str> for WidgetId {
    #[must_use]
    fn from(str: &str) -> Self {
        Self::new(str)
    }
}
