use {
    crate::{gui::WindowSystem, resources::CursorMapPosition},
    bevy::ecs::{
        system::{SystemParam, SystemState},
        world::World,
    },
    bevy_egui::egui::{Grid, Ui},
    planet::{BiomeStats, TerrainCell, WorldManager},
    std::marker::PhantomData,
};

#[derive(SystemParam)]
pub struct TileInfo<'w, 's> {
    #[system_param(ignore)]
    _phantom: PhantomData<(&'w (), &'s ())>,
}

impl WindowSystem for TileInfo<'_, '_> {
    fn draw_contents(world: &mut World, _state: &mut SystemState<Self>, ui: &mut Ui) {
        _ = Grid::new("info_panel")
            .num_columns(2)
            .striped(false)
            .show(ui, |ui| {
                let cursor_position = world.resource::<CursorMapPosition>();
                let cursor_y = cursor_position.y;
                let cursor_x = cursor_position.x;

                let Some(world) = world.resource::<WorldManager>().get_world() else {
                    ui.label("No world.");
                    return;
                };
                if cursor_x >= 0
                    && cursor_x < world.width.try_into().unwrap()
                    && cursor_y >= 0
                    && cursor_y < world.height.try_into().unwrap()
                {
                    let TerrainCell {
                        altitude,
                        rainfall,
                        temperature,
                        biome_presences,
                        x,
                        y,
                    } = &world.terrain[cursor_y as usize][cursor_x as usize];

                    _ = ui.label("Coordinates");
                    _ = ui.label(format!("{x}:{y}"));
                    ui.end_row();
                    _ = ui.label("Altitude");
                    _ = ui.label(format!("{altitude:.2}"));
                    ui.end_row();
                    _ = ui.label("Rainfall");
                    _ = ui.label(format!("{rainfall:.2}"));
                    ui.end_row();
                    _ = ui.label("Temperature");
                    _ = ui.label(format!("{temperature:.2}"));

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

    fn name() -> &'static str {
        "Tile Info"
    }

    fn resizable() -> bool {
        false
    }
}
