use {
    crate::{
        gui::WindowSystem,
        planet_renderer::{WorldOverlay, WorldRenderSettings},
        resources::ShouldRedraw,
    },
    bevy::ecs::{
        change_detection::Mut,
        system::{SystemParam, SystemState},
        world::World,
    },
    bevy_egui::egui::Ui,
    std::marker::PhantomData,
};

#[derive(SystemParam)]
pub struct WorldOverlaySelection<'w, 's> {
    #[system_param(ignore)]
    _phantom: PhantomData<(&'w (), &'s ())>,
}

impl WindowSystem for WorldOverlaySelection<'_, '_> {
    fn draw_contents(world: &mut World, _state: &mut SystemState<Self>, ui: &mut Ui) {
        world.resource_scope(|world, mut render_settings: Mut<WorldRenderSettings>| {
            for overlay in WorldOverlay::iterator() {
                if ui
                    .selectable_label(
                        render_settings.overlay_visible(overlay),
                        <&'static str>::from(overlay),
                    )
                    .clicked()
                {
                    render_settings.toggle_overlay(overlay);
                    world.resource_mut::<ShouldRedraw>().0 = true;
                }
            }
        });
    }

    fn name() -> &'static str {
        "Overlay Selection"
    }

    fn resizable() -> bool {
        false
    }
}
