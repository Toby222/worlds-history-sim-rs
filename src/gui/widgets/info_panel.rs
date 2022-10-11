#[cfg(feature = "logging")]
use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};
use {
    crate::{
        gui::{WidgetId, WidgetSystem},
        resources::CursorMapPosition,
    },
    bevy::ecs::{
        system::{SystemParam, SystemState},
        world::World,
    },
    bevy_egui::egui::{Grid, Ui},
    std::marker::PhantomData,
};

#[derive(SystemParam)]
pub(crate) struct InfoPanel<'w, 's> {
    #[system_param(ignore)]
    _phantom: PhantomData<(&'w (), &'s ())>,
}
impl WidgetSystem for InfoPanel<'_, '_> {
    fn system(world: &mut World, _state: &mut SystemState<Self>, ui: &mut Ui, _id: WidgetId) {
        // This will get everything our system/widget requested
        // let mut params = state.get_mut(world);

        _ = Grid::new("info_panel")
            .num_columns(2)
            .striped(true)
            .show(ui, |ui| {
                #[cfg(feature = "logging")]
                {
                    let diagnostics = world.resource::<Diagnostics>();

                    _ = ui.label("Framerate");
                    _ = ui.label(
                        match diagnostics.get_measurement(FrameTimeDiagnosticsPlugin::FPS) {
                            None => f64::NAN,
                            Some(fps) => fps.value.round(),
                        }
                        .to_string(),
                    );
                    ui.end_row();
                }

                _ = ui.label("Cursor position");
                _ = ui.label(world.resource::<CursorMapPosition>().to_string());
                ui.end_row()
            });
    }
}
