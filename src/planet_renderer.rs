use {
    crate::macros::iterable_enum_stringify,
    bevy::{asset::HandleId, prelude::Color, utils::HashSet},
    planet::{BiomeStats, TerrainCell, World, WorldManager},
};

iterable_enum_stringify!(WorldView {
    Biomes,
    Topography,
    Coastlines
});
iterable_enum_stringify!(WorldOverlay {
    Temperature,
    Rainfall
});

#[cfg(feature = "render")]
#[derive(Debug, Default)]
pub struct WorldRenderSettings {
    pub map_image_handle_id: Option<HandleId>,

    visible_overlays: HashSet<WorldOverlay>,
    pub view:         WorldView,
}

#[cfg(feature = "render")]
impl WorldRenderSettings {
    pub fn overlay_visible(&self, overlay: &WorldOverlay) -> bool {
        self.visible_overlays.contains(overlay)
    }

    pub fn toggle_overlay(&mut self, overlay: &WorldOverlay) {
        if self.visible_overlays.contains(overlay) {
            assert!(
                self.visible_overlays.remove(overlay),
                "Failed to remove overlay [{overlay:#?}], that shouldn't happen."
            );
        } else {
            assert!(
                self.visible_overlays.insert(*overlay),
                "Failed to insert overlay [{overlay:#?}], that shouldn't happen."
            );
        }
    }
}

#[must_use]
fn altitude_contour_color(altitude: f32) -> Color {
    if altitude < 0.0 {
        Color::rgb(0.0, 0.0, (2.0 - altitude / World::MIN_ALTITUDE) / 2.0)
    } else {
        let mut shade_value = 1.0;

        while shade_value > altitude / World::MAX_ALTITUDE {
            shade_value -= 0.05;
        }

        Color::rgb(shade_value, shade_value, shade_value)
    }
}

#[must_use]
fn rainfall_contour_color(world: &World, rainfall: f32) -> Color {
    let mut shade_value = 1.0;
    let value = rainfall / world.max_rainfall;

    while shade_value > value {
        shade_value -= 0.1;
    }

    Color::rgb(0.0, shade_value, 0.0)
}

#[must_use]
fn temperature_contour_color(world: &World, temperature: f32) -> Color {
    let mut shade_value = 1.0;
    let value =
        (temperature - world.min_temperature) / (world.max_temperature - world.min_temperature);

    while shade_value > value {
        shade_value -= 0.1;
    }

    Color::rgb(shade_value, 0.0, 1.0 - shade_value)
}

#[must_use]
fn biome_color(world: &World, cell: &TerrainCell) -> Color {
    let slant = world.get_slant(cell);

    let slant_factor = f32::min(1.0, (4.0 + (10.0 * slant / World::ALTITUDE_SPAN)) / 5.0);
    let altitude_factor = f32::min(
        1.0,
        (0.5 + (cell.altitude - World::MIN_ALTITUDE) / World::ALTITUDE_SPAN) / 1.5,
    );

    let mut red = 0.0;
    let mut green = 0.0;
    let mut blue = 0.0;

    for (biome, presence) in cell.biome_presences.iter() {
        red += BiomeStats::from(biome).color.r() * presence;
        green += BiomeStats::from(biome).color.g() * presence;
        blue += BiomeStats::from(biome).color.b() * presence;
    }
    red *= slant_factor * altitude_factor;
    green *= slant_factor * altitude_factor;
    blue *= slant_factor * altitude_factor;
    Color::rgb(red, green, blue)
}

#[must_use]
fn coastline_color(world: &World, cell: &TerrainCell) -> Color {
    if world.is_cell_coastline(cell) {
        Color::BLACK
    } else if cell.altitude > 0.0 {
        Color::rgb(0.75, 0.75, 0.75)
    } else {
        Color::ANTIQUE_WHITE
    }
}
pub(crate) trait WorldRenderer {
    fn map_color_bytes(&self, render_settings: &WorldRenderSettings) -> Vec<u8>;
    fn generate_color(&self, cell: &TerrainCell, render_settings: &WorldRenderSettings) -> Color;
}
impl WorldRenderer for WorldManager {
    #[must_use]
    fn map_color_bytes(&self, render_settings: &WorldRenderSettings) -> Vec<u8> {
        self.world()
            .terrain
            .iter()
            .rev()
            .flatten()
            .flat_map(|cell| {
                self.generate_color(cell, render_settings)
                    .as_rgba_f32()
                    .iter()
                    .flat_map(|num| num.to_le_bytes())
                    .collect::<Vec<u8>>()
            })
            .collect()
    }

    #[must_use]
    fn generate_color(&self, cell: &TerrainCell, render_settings: &WorldRenderSettings) -> Color {
        let base_color = match render_settings.view {
            WorldView::Biomes => biome_color(&self.world(), cell),
            WorldView::Topography => altitude_contour_color(cell.altitude),
            WorldView::Coastlines => coastline_color(&self.world(), cell),
        };
        let mut normalizer = 1.0;

        let mut red = base_color.r();
        let mut green = base_color.g();
        let mut blue = base_color.b();

        if render_settings.overlay_visible(&WorldOverlay::Rainfall) {
            normalizer += 1.0;
            let rainfall_color = rainfall_contour_color(self.world(), cell.rainfall);

            red += rainfall_color.r();
            green += rainfall_color.g();
            blue += rainfall_color.b();
        }

        if render_settings.overlay_visible(&WorldOverlay::Temperature) {
            normalizer += 1.0;
            let temperature_color = temperature_contour_color(self.world(), cell.temperature);

            red += temperature_color.r();
            green += temperature_color.g();
            blue += temperature_color.b();
        }

        Color::rgb(red / normalizer, green / normalizer, blue / normalizer)
    }
}

impl Default for WorldView {
    fn default() -> Self {
        WorldView::Biomes
    }
}
