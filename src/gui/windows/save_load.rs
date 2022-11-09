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

                ui.horizontal(|ui| {
                    if ui.button("Save").clicked() {
                        if let Some(path) = tinyfiledialogs::save_file_dialog_with_filter(
                            "Save world",
                            state.file_name.as_str(),
                            &["*.rsplnt", "*.ron"],
                            "World file",
                        ) {
                            if let Err(err) = world_manager.save_world(&path) {
                                // TODO: Error popup
                                error!("Failed to save: {err:#?}");
                            }
                            *state.file_name = path;
                        }
                    }
                    if ui.button("Load").clicked() {
                        if let Some(path) = tinyfiledialogs::open_file_dialog(
                            "World file",
                            state.file_name.as_str(),
                            Some((&["*.ron", "*.rsplnt"], "*.ron,*.rsplnt")),
                        ) {
                            if let Err(err) = world_manager.load_world(&path) {
                                // TODO: Error popup
                                error!("Failed to load: {err:#?}");
                            }
                            *state.file_name = path;
                            should_redraw.0 = true;
                        }
                    }
                });
            });
        });
    }

    fn name() -> &'static str {
        "Save/Load world"
    }

    fn resizable() -> bool {
        true
    }
}
