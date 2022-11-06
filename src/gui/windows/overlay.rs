use {
    crate::gui::WindowSystem,
    bevy::ecs::{
        system::{SystemParam, SystemState},
        world::World,
    },
    bevy_egui::egui::Ui,
    std::marker::PhantomData,
};

#[derive(SystemParam)]
pub(crate) struct Overlay<'w, 's> {
    #[system_param(ignore)]
    _phantom: PhantomData<(&'w (), &'s ())>,
}

impl WindowSystem for Overlay<'_, '_> {
    fn draw_contents(world: &mut World, _state: &mut SystemState<Self>, ui: &mut Ui) {
        ui.label(format!("{world:#?}"));
    }

    fn name() -> &'static str {
        "Overlay Selection"
    }
}
