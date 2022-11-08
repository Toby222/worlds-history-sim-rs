use {
    crate::{gui::WindowSystem, resources::ShouldRedraw},
    bevy::{
        ecs::{
            change_detection::Mut,
            system::{Local, SystemParam, SystemState},
            world::World,
        },
        log::error,
    },
    bevy_egui::egui::Ui,
    planet::WorldManager,
    std::marker::PhantomData,
};

#[derive(SystemParam)]
pub(crate) struct SaveLoad<'w, 's> {
    pub file_name: Local<'s, String>,
    #[system_param(ignore)]
    _phantom:      PhantomData<(&'w (), &'s ())>,
}

impl WindowSystem for SaveLoad<'_, '_> {
    fn draw_contents(world: &mut World, state: &mut SystemState<Self>, ui: &mut Ui) {
        world.resource_scope(|world, mut world_manager: Mut<WorldManager>| {
            world.resource_scope(|world, mut should_redraw: Mut<ShouldRedraw>| {
                let mut state = state.get_mut(world);

                // TODO: Real file selection dialog.
                ui.text_edit_singleline(&mut *state.file_name);

                ui.horizontal(|ui| {
                    if ui.button("Save").clicked() {
                        if let Err(err) = world_manager.save_world(&*state.file_name) {
                            // TODO: Error popup
                            error!("Failed to save: {err:#?}");
                        }
                    }
                    if ui.button("Load").clicked() {
                        if let Err(err) = world_manager.load_world(&*state.file_name) {
                            // TODO: Error popup
                            error!("Failed to load: {err:#?}");
                        }
                        should_redraw.0 = true;
                    }
                });
            });
        });
    }

    fn name() -> &'static str {
        "Save/Load world"
    }
}
