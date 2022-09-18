#[cfg(feature = "render")]
use crate::TerrainCell;
use crate::{Biome, World, WorldGenError};
#[cfg(all(feature = "debug", feature = "render"))]
use bevy::log::debug;
use bevy::log::warn;
#[cfg(feature = "debug")]
use bevy::utils::default;
#[cfg(feature = "render")]
use bevy::{
    asset::{Assets, HandleId},
    render::render_resource::Extent3d,
    render::{color::Color, texture::Image},
};
use rand::random;
use std::{
    error::Error,
    fmt::Display,
    fs::File,
    io::{self, Read, Write},
    path::Path,
};

#[derive(Debug)]
pub enum LoadError {
    MissingSave(io::Error),
    InvalidSave(ron::Error),
}
impl Error for LoadError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            LoadError::MissingSave(error) => Some(error),
            LoadError::InvalidSave(error) => Some(error),
        }
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}
impl Display for LoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoadError::MissingSave(_) => f.write_str("No save found at given path"),
            LoadError::InvalidSave(_) => f.write_str("Loaded file is not a valid save"),
        }
    }
}

#[derive(Debug)]
pub enum SaveError {
    MissingWorld,
    SerializationError(ron::Error),
    FailedToWrite(io::Error),
}
impl Error for SaveError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            SaveError::MissingWorld => None,
            SaveError::SerializationError(error) => Some(error),
            SaveError::FailedToWrite(err) => Some(err),
        }
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}
impl Display for SaveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SaveError::MissingWorld => f.write_str("No world to save found."),
            SaveError::SerializationError(_) => f.write_str("Failed to serialize world."),
            SaveError::FailedToWrite(_) => f.write_str("Failed to write save file."),
        }
    }
}

#[derive(Debug)]
pub struct WorldManager {
    #[cfg(feature = "render")]
    pub image_handle_id: Option<HandleId>,
    world: Option<World>,
    #[cfg(feature = "render")]
    rainfall_visible: bool,
    #[cfg(feature = "render")]
    temperature_visible: bool,
    #[cfg(feature = "render")]
    biomes_visible: bool,
    #[cfg(feature = "render")]
    contours: bool,
}

impl WorldManager {
    pub fn new() -> WorldManager {
        Self {
            #[cfg(feature = "render")]
            image_handle_id: None,
            world: None,
            #[cfg(feature = "render")]
            rainfall_visible: false,
            #[cfg(feature = "render")]
            temperature_visible: false,
            #[cfg(feature = "render")]
            biomes_visible: false,
            #[cfg(feature = "render")]
            contours: true,
        }
    }

    pub fn save_world<P: AsRef<Path>>(&self, path: P) -> Result<(), SaveError> {
        let world = match self.get_world() {
            Some(world) => world,
            None => {
                warn!("No world to save");
                return Err(SaveError::MissingWorld);
            }
        };
        #[cfg(feature = "debug")]
        let serialized = match ron::ser::to_string_pretty(world, default()) {
            Ok(serialized) => serialized,
            Err(err) => {
                return Err(SaveError::SerializationError(err));
            }
        };

        #[cfg(not(feature = "debug"))]
        let serialized = match ron::to_string(world) {
            Ok(serialized) => serialized,
            Err(err) => {
                return Err(SaveError::SerializationError(err));
            }
        };

        match File::create(path).unwrap().write_all(serialized.as_bytes()) {
            Ok(_) => Ok(()),
            Err(err) => Err(SaveError::FailedToWrite(err)),
        }
    }

    pub fn load_world<P: AsRef<Path>>(
        &mut self,
        path: P,
        #[cfg(feature = "render")] images: &mut Assets<Image>,
    ) -> Result<(), LoadError> {
        let mut file = match File::open(path) {
            Ok(file) => file,
            Err(err) => {
                return Err(LoadError::MissingSave(err));
            }
        };
        let mut buf = String::new();
        match file.read_to_string(&mut buf) {
            Ok(_) => {}
            Err(err) => {
                return Err(LoadError::MissingSave(err));
            }
        };
        match ron::from_str(buf.as_str()) {
            Ok(world) => {
                #[cfg(feature = "render")]
                let World { height, width, .. } = world;
                self.world = Some(world);
                #[cfg(feature = "render")]
                {
                    let image_handle = &images.get_handle(
                        self.image_handle_id
                            .expect("Missing image handle, even though world is present"),
                    );
                    images
                        .get_mut(image_handle)
                        .expect("Handle for missing image")
                        .resize(Extent3d {
                            width,
                            height,
                            depth_or_array_layers: 0,
                        });
                }
                Ok(())
            }
            Err(err) => Err(LoadError::InvalidSave(err)),
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
    pub fn toggle_biomes(&mut self) {
        #[cfg(feature = "debug")]
        if self.temperature_visible {
            debug!("Turning biomes off");
        } else {
            debug!("Turning biomes on");
        }
        self.biomes_visible = !self.biomes_visible;
    }

    #[cfg(feature = "render")]
    pub fn toggle_contours(&mut self) {
        #[cfg(feature = "debug")]
        if self.contours {
            debug!("Turning terrain contours off");
        } else {
            debug!("Turning terrain contours on");
        }
        self.contours = !self.contours;
    }

    pub fn get_world(&self) -> Option<&World> {
        self.world.as_ref()
    }
    pub fn world(&self) -> &World {
        assert!(self.world.is_some(), "No world.");
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
        if self.biomes_visible {
            return WorldManager::biome_color(cell);
        }

        let altitude_color = if self.contours {
            WorldManager::altitude_contour_color(cell.altitude)
        } else {
            WorldManager::altitude_color(cell.altitude)
        };

        let mut layer_count = 1.0;

        let mut red = altitude_color.r();
        let mut green = altitude_color.g();
        let mut blue = altitude_color.b();

        if self.rainfall_visible {
            layer_count += 1.0;
            let rainfall_color = self.rainfall_contour_color(cell.rainfall);
            // if self.contours {
            //     self.rainfall_contour_color(cell.rainfall)
            // } else {
            //     WorldManager::rainfall_color(cell.rainfall)
            // };

            red += rainfall_color.r();
            green += rainfall_color.g();
            blue += rainfall_color.b();
        }

        if self.temperature_visible {
            layer_count += 1.0;
            let temperature_color = self.temperature_contour_color(cell.temperature);
            // if self.contours {
            //     self.temperature_contour_color(cell.temperature)
            // } else {
            //     WorldManager::temperature_color(cell.temperature)
            // };

            red += temperature_color.r();
            green += temperature_color.g();
            blue += temperature_color.b();
        }

        Color::rgb(red / layer_count, green / layer_count, blue / layer_count)
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
    fn rainfall_contour_color(&self, rainfall: f32) -> Color {
        let mut shade_value = 1.0;
        let value = rainfall / self.world().max_rainfall;

        while shade_value > value {
            shade_value -= 0.1;
        }

        Color::rgb(0.0, shade_value, 0.0)
    }

    #[cfg(feature = "render")]
    fn temperature_contour_color(&self, temperature: f32) -> Color {
        let mut shade_value = 1.0;
        let value = (temperature - self.world().min_temperature)
            / (self.world().max_temperature - self.world().min_temperature);

        while shade_value > value {
            shade_value -= 0.1;
        }

        Color::rgb(shade_value, 0.0, 1.0 - shade_value)
    }

    #[cfg(feature = "render")]
    fn biome_color(cell: &TerrainCell) -> Color {
        cell.biome_presences
            .iter()
            .fold(Color::BLACK, |color, (biome_type, presence)| {
                let biome: Biome = (*biome_type).into();
                let biome_color = biome.color;

                Color::rgb(
                    color.r() + (biome_color.r() * presence),
                    color.g() + (biome_color.g() * presence),
                    color.b() + (biome_color.b() * presence),
                )
            })
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
