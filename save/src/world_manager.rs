#[cfg(feature = "render")]
use crate::TerrainCell;
use crate::{World, WorldGenError};
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
}

impl WorldManager {
    pub fn new() -> WorldManager {
        Self {
            #[cfg(feature = "render")]
            image_handle_id: HandleId::default::<Image>(),
            world: None,
            rainfall_visible: false,
        }
    }

    #[cfg(feature = "render")]
    pub fn toggle_rainfall(&mut self) {
        if self.rainfall_visible {
            debug!("Turning rainfall off");
        } else {
            debug!("Turning rainfall on");
            debug!("World: {:#?}", self.world);
        }
        self.rainfall_visible = !self.rainfall_visible;
    }

    pub fn get_world(&self) -> Option<&World> {
        self.world.as_ref()
    }

    pub fn new_world(&mut self) -> Result<&World, WorldGenError> {
        let seed = random();
        let mut new_world = World::new(400, 200, seed);
        new_world.generate()?;
        self.world = Some(new_world);
        Ok(self.get_world().unwrap())
    }

    #[cfg(feature = "render")]
    fn generate_color(cell: &TerrainCell, show_rainfall: bool) -> Color {
        let altitude_color = Self::altitude_contour_color(cell.altitude);
        let rainfall_color = if show_rainfall {
            Self::rainfall_color(cell.rainfall)
        } else {
            Color::BLACK
        };

        let normalized_rainfall = Self::normalize_rainfall(cell.rainfall);

        let r = (altitude_color.r() * (1.0 - normalized_rainfall))
            + (rainfall_color.r() * normalized_rainfall);
        let g = (altitude_color.g() * (1.0 - normalized_rainfall))
            + (rainfall_color.g() * normalized_rainfall);
        let b = (altitude_color.b() * (1.0 - normalized_rainfall))
            + (rainfall_color.b() * normalized_rainfall);

        Color::rgb_linear(r, g, b)
    }

    /*
    #[cfg(feature = "render")]
    fn altitude_color(altitude: f32) -> Color {
        if altitude < 0.0 {
            Color::rgb(0.0, 0.0, (2.0 - altitude / World::MIN_ALTITUDE) / 2.0)
        } else {
            let mult = (1.0 + altitude / World::MAX_ALTITUDE) / 2.0;

            Color::rgb(0.58 * mult, 0.29 * mult, 0.0)
        }
    }
    */

    #[cfg(feature = "render")]
    fn altitude_contour_color(altitude: f32) -> Color {
        if altitude < 0.0 {
            Color::rgb(0.0, 0.0, (2.0 - altitude / World::MIN_ALTITUDE) / 2.0)
        } else {
            let mut shade_value = 1.0;

            while shade_value > altitude / World::MAX_ALTITUDE {
                shade_value -= 0.1;
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
    pub fn world_colors(&self) -> Vec<Color> {
        match self.get_world() {
            None => panic!("Called world_colors before generating world"),
            Some(world) => {
                let terrain_cells: Vec<_> = world.terrain.iter().rev().flatten().collect();

                terrain_cells
                    .iter()
                    .map(|cell| Self::generate_color(cell, self.rainfall_visible))
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
