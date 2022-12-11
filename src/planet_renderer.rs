use {
    crate::macros::iterable_enum_stringify,
    bevy::{asset::HandleId, prelude::*, utils::HashSet},
    planet::{BiomeStats, TerrainCell, World, WorldManager},
};

iterable_enum_stringify!(WorldView {
    Population,
    Biomes,
    Topography,
    Coastlines,
});
impl Default for WorldView {
    fn default() -> Self {
        WorldView::Population
    }
}

iterable_enum_stringify!(WorldOverlay {
    Temperature,
    Rainfall,
});

#[cfg(feature = "render")]
#[derive(Debug, Default, Resource)]
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
fn population_color(world: &World, cell: &TerrainCell) -> Color {
    let slant = world.get_slant(cell);
    let altitude_difference = world.max_altitude - world.min_altitude;

    let slant_factor = f32::min(1.0, (4.0 + (10.0 * slant / altitude_difference)) / 5.0);

    let altitude_factor = f32::min(1.0, (0.5 + ((cell.altitude - altitude_difference) / altitude_difference)) / 1.5);

    let mut total_population = 0;

    for human_group in cell.human_groups {
        total_population += human_group.population;
    }

    let color = if total_population > 0 {
        Color::GREEN
    } else {
        let color = biome_color(world, cell);

        let greyscale = (color.r() + color.g() + color.b()) / 4.5 + 0.25;

        color.set_r(greyscale);
        color.set_g(greyscale);
        color.set_b(greyscale);

        color
    };

    let r = color.r() * slant_factor * altitude_factor;
    let g = color.g() * slant_factor * altitude_factor;
    let b = color.b() * slant_factor * altitude_factor;

    Color::rgb(r, g, b)
}

#[must_use]
fn altitude_contour_color(world: &World, altitude: f32) -> Color {
    let mut color = Color::rgb(1.0, 0.6, 0.0);

    let mut shade_value = 1.0;
    let mut value = f32::max(0.0, altitude / world.max_altitude);

    if altitude < 0.0 {
        value = f32::max(0.0, 1.0 - (altitude / world.min_altitude));
        color = Color::BLUE;
    }

    while shade_value > value {
        shade_value -= 0.15;
    }
    shade_value = 0.5 * shade_value + 0.5;

    Color::rgb(
        color.r() * shade_value,
        color.g() * shade_value,
        color.b() * shade_value,
    )
}

#[cfg(feature = "placeholder")]
#[must_use]
fn rainfall_contour_color(world: &World, rainfall: f32) -> Color {
    let mut shade_value = 1.0;
    let value = f32::max(0.0, rainfall / world.max_rainfall);

    while shade_value > value {
        shade_value -= 0.1;
    }

    Color::rgb(0.0, shade_value, 0.0)
}

#[must_use]
fn rainfall_color(rainfall: f32) -> Color {
    if rainfall <= 0.0 {
        Color::BLACK
    } else {
        Color::rgb(0.0, rainfall / World::MAX_RAINFALL, 0.0)
    }
}

#[cfg(feature = "placeholder")]
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
fn temperature_color(temperature: f32) -> Color {
    let value = (temperature - World::MIN_TEMPERATURE) / World::TEMPERATURE_SPAN;
    Color::rgb(value, 0.0, 1.0 - value)
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

const COASTLINE_PALETTE: [Color; 5] = [
    Color::rgb(251.0 / 255.0, 230.0 / 255.0, 197.0 / 255.0),
    Color::rgb(233.0 / 255.0, 192.0 / 255.0, 136.0 / 255.0),
    Color::rgb(74.0 / 255.0, 39.0 / 255.0, 13.0 / 255.0),
    Color::rgb(155.0 / 255.0, 105.0 / 255.0, 72.0 / 255.0),
    Color::rgb(188.0 / 255.0, 136.0 / 255.0, 84.0 / 255.0),
];
#[must_use]
fn coastline_color(world: &World, cell: &TerrainCell) -> Color {
    if world.is_cell_coastline(cell) {
        COASTLINE_PALETTE[2]
    } else if world.is_cell_near_coastline(cell) {
        COASTLINE_PALETTE[3]
    } else if cell.altitude > 0.0 {
        let slant = world.get_slant(cell);
        let altitude_span = world.max_altitude - world.min_altitude;
        let slant_factor = f32::min(1.0, -(20.0 * (slant / altitude_span)));

        if slant_factor > 0.1 {
            COASTLINE_PALETTE[4] * slant_factor + COASTLINE_PALETTE[1] * (1.0 - slant_factor)
        } else {
            COASTLINE_PALETTE[1]
        }
    } else {
        COASTLINE_PALETTE[0]
    }
}

const CACHE_SIZE: usize =
    WorldOverlay::ITEM_COUNT * WorldOverlay::ITEM_COUNT * WorldView::ITEM_COUNT;

#[derive(Default, Resource)]
pub struct WorldRenderer {
    cached_world_seed: u32,
    cache:             [Option<Vec<u8>>; CACHE_SIZE],
}

impl WorldRenderer {
    fn cache_index(render_settings: &WorldRenderSettings) -> usize {
        let view_num = match render_settings.view {
            WorldView::Biomes => 0,
            WorldView::Topography => 1,
            WorldView::Coastlines => 2,
        };
        let mut overlay_num = 0;
        for overlay in render_settings.visible_overlays.iter() {
            overlay_num |= match overlay {
                WorldOverlay::Temperature => 1,
                WorldOverlay::Rainfall => 2,
            };
        }
        #[cfg(feature = "logging")]
        debug!(overlay_num, view_num);
        (view_num << WorldOverlay::ITEM_COUNT) | overlay_num
    }

    #[must_use]
    pub fn map_color_bytes(
        &mut self,
        world_manager: &WorldManager,
        render_settings: &WorldRenderSettings,
    ) -> Vec<u8> {
        let Some(world) = world_manager.get_world() else {
            return vec![];
        };

        let cache_index = WorldRenderer::cache_index(render_settings);
        #[cfg(feature = "logging")]
        debug!(cache_index);
        assert!(
            cache_index < CACHE_SIZE,
            "Generated cache index too large for render cache"
        );
        if world.seed != self.cached_world_seed {
            self.cache = default();
            self.cached_world_seed = world.seed;
        }
        if let Some(cached) = &self.cache[cache_index] {
            return cached.clone();
        }

        let bytes: Vec<_> = world
            .terrain
            .iter()
            .rev()
            .flatten()
            .flat_map(|cell| {
                self.generate_color(world_manager, cell, render_settings)
                    .as_rgba_f32()
                    .iter()
                    .flat_map(|num| num.to_le_bytes())
                    .collect::<Vec<u8>>()
            })
            .collect();
        let result = bytes.clone();
        self.cache[cache_index] = Some(bytes);

        result
    }

    #[must_use]
    pub fn generate_color(
        &self,
        world_manager: &WorldManager,
        cell: &TerrainCell,
        render_settings: &WorldRenderSettings,
    ) -> Color {
        let world = world_manager
            .get_world()
            .expect("No world in generate_color");
        let base_color = match render_settings.view {
            WorldView::Biomes => biome_color(world, cell),
            WorldView::Topography => altitude_contour_color(world, cell.altitude),
            WorldView::Coastlines => coastline_color(world, cell),
        };
        let mut normalizer = 1.0;

        let mut red = base_color.r();
        let mut green = base_color.g();
        let mut blue = base_color.b();

        if render_settings.overlay_visible(&WorldOverlay::Rainfall)
            || render_settings.overlay_visible(&WorldOverlay::Temperature)
        {
            let grey = (red + green + blue) / 3.0;
            red = grey;
            green = grey;
            blue = grey;
        }

        if render_settings.overlay_visible(&WorldOverlay::Rainfall) {
            normalizer += 1.0;
            let rainfall_color = rainfall_color(cell.rainfall);

            red += rainfall_color.r();
            green += rainfall_color.g();
            blue += rainfall_color.b();
        }

        if render_settings.overlay_visible(&WorldOverlay::Temperature) {
            normalizer += 1.0;
            let temperature_color = temperature_color(cell.temperature);

            red += temperature_color.r();
            green += temperature_color.g();
            blue += temperature_color.b();
        }

        Color::rgb(red / normalizer, green / normalizer, blue / normalizer)
    }
}
