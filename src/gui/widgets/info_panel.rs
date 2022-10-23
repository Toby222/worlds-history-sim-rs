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
    planet::{BiomeStats, TerrainCell, WorldManager},
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
            .striped(false)
            .show(ui, |ui| {
                let cursor_position = world.resource::<CursorMapPosition>();
                let cursor_y = cursor_position.y;
                let cursor_x = cursor_position.x;
                _ = ui.label("Coordinates");
                _ = ui.label(cursor_position.to_string());
                ui.end_row();

                let world_manager = world.resource::<WorldManager>();
                if cursor_x >= 0
                    && cursor_x <= world_manager.world().width.try_into().unwrap()
                    && cursor_y >= 0
                    && cursor_y < world_manager.world().height.try_into().unwrap()
                {
                    let TerrainCell {
                        altitude,
                        rainfall,
                        temperature,
                        biome_presences,
                    } = &world_manager.world().terrain[cursor_y as usize][cursor_x as usize];

                    _ = ui.label("Altitude");
                    _ = ui.label(format!("{:.2}", altitude));
                    ui.end_row();
                    _ = ui.label("Rainfall");
                    _ = ui.label(format!("{:.2}", rainfall));
                    ui.end_row();
                    _ = ui.label("Temperature");
                    _ = ui.label(format!("{:.2}", temperature));

                    ui.end_row();
                    ui.end_row();
                    _ = ui.label("Biome presences");
                    for (biome_type, presence) in biome_presences {
                        ui.end_row();
                        _ = ui.label(<BiomeStats>::from(biome_type).name);
                        _ = ui.label(format!("{:.2}%", presence * 100.0));
                    }
                } else {
                    _ = ui.label("No tile at this position");
                }
            });
    }
}
