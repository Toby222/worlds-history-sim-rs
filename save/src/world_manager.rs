#[cfg(feature = "render")]
use crate::TerrainCell;
use crate::{World, WorldGenError};
#[cfg(all(feature = "debug", feature = "render"))]
use bevy::log::debug;
#[cfg(feature = "render")]
use bevy::{
    asset::HandleId,
    render::{color::Color, texture::Image},
};
use rand::random;

#[derive(Debug)]
pub struct WorldManager {
    #[cfg(feature = "render")]
    pub image_handle_id: HandleId,
    world: Option<World>,
    #[cfg(feature = "render")]
    rainfall_visible: bool,
    #[cfg(feature = "render")]
    temperature_visible: bool,
    #[cfg(feature = "render")]
    terrain_as_contours: bool,
}

impl WorldManager {
    pub fn new() -> WorldManager {
        Self {
            #[cfg(feature = "render")]
            image_handle_id: HandleId::default::<Image>(),
            world: None,
            #[cfg(feature = "render")]
            rainfall_visible: false,
            #[cfg(feature = "render")]
            temperature_visible: false,
            #[cfg(feature = "render")]
            terrain_as_contours: false,
        }
    }

    #[cfg(feature = "render")]
    pub fn toggle_rainfall(&mut self) {
        #[cfg(feature = "debug")]
        if self.rainfall_visible {
            debug!("Turning rainfall off");
        } else {
            debug!("Turning rainfall on");
        }
        self.rainfall_visible = !self.rainfall_visible;
    }

    #[cfg(feature = "render")]
    pub fn toggle_temperature(&mut self) {
        #[cfg(feature = "debug")]
        if self.temperature_visible {
            debug!("Turning temperature off");
        } else {
            debug!("Turning temperature on");
        }
        self.temperature_visible = !self.temperature_visible;
    }

    #[cfg(feature = "render")]
    pub fn toggle_contours(&mut self) {
        #[cfg(feature = "debug")]
        if self.terrain_as_contours {
            debug!("Turning terrain contours off");
        } else {
            debug!("Turning terrain contours on");
        }
        self.terrain_as_contours = !self.terrain_as_contours;
    }

    pub fn get_world(&self) -> Option<&World> {
        self.world.as_ref()
    }
    pub fn world(&self) -> &World {
        self.get_world().unwrap()
    }

    pub fn new_world(&mut self) -> Result<&World, WorldGenError> {
        let seed = random();
        let mut new_world = World::new(400, 200, seed);
        new_world.generate()?;
        self.world = Some(new_world);
        Ok(self.get_world().unwrap())
    }

    #[cfg(feature = "render")]
    fn generate_color(&self, cell: &TerrainCell) -> Color {
        let mut final_color = if self.terrain_as_contours {
            Self::altitude_contour_color(cell.altitude)
        } else {
            Self::altitude_color(cell.altitude)
        };

        if self.rainfall_visible {
            let rainfall_color = Self::rainfall_color(cell.rainfall);
            let normalized_rainfall = Self::normalize_rainfall(cell.rainfall);

            _ = final_color.set_r(
                (final_color.r() * (1.0 - normalized_rainfall))
                    + (rainfall_color.r() * normalized_rainfall),
            );
            _ = final_color.set_g(
                (final_color.g() * (1.0 - normalized_rainfall))
                    + (rainfall_color.g() * normalized_rainfall),
            );
            _ = final_color.set_b(
                (final_color.b() * (1.0 - normalized_rainfall))
                    + (rainfall_color.b() * normalized_rainfall),
            );
        }

        if self.temperature_visible {
            let temperature_color = Self::temperature_color(cell.temperature);
            let normalized_temperature = Self::normalize_temperature(cell.temperature);

            _ = final_color.set_r(
                (final_color.r() * (1.0 - normalized_temperature))
                    + (temperature_color.r() * normalized_temperature),
            );
            _ = final_color.set_g(
                (final_color.g() * (1.0 - normalized_temperature))
                    + (temperature_color.g() * normalized_temperature),
            );
            _ = final_color.set_b(
                (final_color.b() * (1.0 - normalized_temperature))
                    + (temperature_color.b() * normalized_temperature),
            );
        }

        final_color
    }

    #[cfg(feature = "render")]
    fn altitude_color(altitude: f32) -> Color {
        if altitude < 0.0 {
            Color::rgb(0.0, 0.0, (2.0 - altitude / World::MIN_ALTITUDE) / 2.0)
        } else {
            let mult = (1.0 + altitude / World::MAX_ALTITUDE) / 2.0;

            Color::rgb(0.58 * mult, 0.29 * mult, 0.0)
        }
    }

    #[cfg(feature = "render")]
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

    #[cfg(feature = "render")]
    fn rainfall_color(rainfall: f32) -> Color {
        Color::rgb(0.0, Self::normalize_rainfall(rainfall), 0.0)
    }

    #[cfg(feature = "render")]
    fn normalize_rainfall(rainfall: f32) -> f32 {
        if rainfall <= 0.0 {
            0.0
        } else {
            rainfall / World::MAX_RAINFALL
        }
    }

    #[cfg(feature = "render")]
    fn temperature_color(temperature: f32) -> Color {
        let normalized_temperature = Self::normalize_temperature(temperature);
        Color::rgb(normalized_temperature, 1.0 - normalized_temperature, 0.0)
    }

    #[cfg(feature = "render")]
    fn normalize_temperature(temperature: f32) -> f32 {
        (temperature - World::MIN_TEMPERATURE) / World::TEMPERATURE_SPAN
    }

    #[cfg(feature = "render")]
    pub fn world_colors(&self) -> Vec<Color> {
        match self.get_world() {
            None => panic!("Called world_colors before generating world"),
            Some(world) => {
                let terrain_cells: Vec<_> = world.terrain.iter().rev().flatten().collect();

                terrain_cells
                    .iter()
                    .map(|cell| self.generate_color(cell))
                    .collect()
            }
        }
    }

    #[cfg(feature = "render")]
    pub fn world_color_bytes(&self) -> Vec<u8> {
        self.world_colors()
            .iter()
            .flat_map(|color| {
                color
                    .as_rgba_f32()
                    .iter()
                    .flat_map(|num| num.to_le_bytes())
                    .collect::<Vec<u8>>()
            })
            .collect()
    }
}
