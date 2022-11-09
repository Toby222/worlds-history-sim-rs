#[cfg(feature = "render")]
use {
    crate::{BiomeStats, TerrainCell, WorldOverlay, WorldRenderSettings, WorldView},
    bevy::render::color::Color,
};
use {
    crate::{World, WorldGenError},
    bevy::{log::warn, utils::default},
    rand::random,
    std::{
        error::Error,
        fmt::Display,
        fs::File,
        io::{self, Read, Write},
        path::Path,
    },
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

#[derive(Debug, Default)]
pub struct WorldManager {
    world: Option<World>,
}

impl WorldManager {
    #[must_use]
    pub fn new() -> WorldManager {
        default()
    }

    pub fn save_world<P: AsRef<Path>>(&self, path: P) -> Result<(), SaveError> {
        let world = match self.get_world() {
            Some(world) => world,
            None => {
                warn!("No world to save");
                return Err(SaveError::MissingWorld);
            },
        };
        #[cfg(feature = "logging")]
        let serialized = match ron::ser::to_string_pretty(world, default()) {
            Ok(serialized) => serialized,
            Err(err) => {
                return Err(SaveError::SerializationError(err));
            },
        };

        #[cfg(not(feature = "logging"))]
        let serialized = match ron::to_string(world) {
            Ok(serialized) => serialized,
            Err(err) => {
                return Err(SaveError::SerializationError(err));
            },
        };

        match File::create(path).unwrap().write_all(serialized.as_bytes()) {
            Ok(_) => Ok(()),
            Err(err) => Err(SaveError::FailedToWrite(err)),
        }
    }

    pub fn load_world<P: AsRef<Path>>(&mut self, path: P) -> Result<(), LoadError> {
        let mut file = match File::open(path) {
            Ok(file) => file,
            Err(err) => {
                return Err(LoadError::MissingSave(err));
            },
        };
        let mut buf = String::new();
        match file.read_to_string(&mut buf) {
            Ok(_) => {},
            Err(err) => {
                return Err(LoadError::MissingSave(err));
            },
        };
        match ron::from_str(buf.as_str()) {
            Ok(world) => {
                self.world = Some(world);
                Ok(())
            },
            Err(err) => Err(LoadError::InvalidSave(err)),
        }
    }

    // #[cfg(feature = "render")]
    // pub fn toggle_contours(&mut self) {
    //     #[cfg(feature = "logging")]
    //     if self.contours {
    //         debug!("Turning terrain contours off");
    //     } else {
    //         debug!("Turning terrain contours on");
    //     }
    //     self.contours = !self.contours;
    // }

    #[must_use]
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
    #[must_use]
    fn generate_color(&self, cell: &TerrainCell, render_settings: &WorldRenderSettings) -> Color {
        let base_color = match render_settings.view {
            WorldView::Biomes => self.biome_color(cell),
            WorldView::Topography => WorldManager::altitude_contour_color(cell.altitude),
        };
        let mut normalizer = 1.0;

        let mut red = base_color.r();
        let mut green = base_color.g();
        let mut blue = base_color.b();

        if render_settings.overlay_visible(&WorldOverlay::Rainfall) {
            normalizer += 1.0;
            let rainfall_color = self.rainfall_contour_color(cell.rainfall);

            red += rainfall_color.r();
            green += rainfall_color.g();
            blue += rainfall_color.b();
        }

        if render_settings.overlay_visible(&WorldOverlay::Temperature) {
            normalizer += 1.0;
            let temperature_color = self.temperature_contour_color(cell.temperature);

            red += temperature_color.r();
            green += temperature_color.g();
            blue += temperature_color.b();
        }

        Color::rgb(red / normalizer, green / normalizer, blue / normalizer)
    }

    // #[cfg(feature = "render")]
    // #[must_use]
    // fn altitude_color(altitude: f32) -> Color {
    //     if altitude < 0.0 {
    //         Color::rgb(0.0, 0.0, (2.0 - altitude / World::MIN_ALTITUDE) / 2.0)
    //     } else {
    //         let mult = (1.0 + altitude / World::MAX_ALTITUDE) / 2.0;

    //         Color::rgb(0.58 * mult, 0.29 * mult, 0.0)
    //     }
    // }

    #[cfg(feature = "render")]
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

    #[cfg(feature = "render")]
    #[must_use]
    fn rainfall_contour_color(&self, rainfall: f32) -> Color {
        let mut shade_value = 1.0;
        let value = rainfall / self.world().max_rainfall;

        while shade_value > value {
            shade_value -= 0.1;
        }

        Color::rgb(0.0, shade_value, 0.0)
    }

    #[cfg(feature = "render")]
    #[must_use]
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
    #[must_use]
    fn biome_color(&self, cell: &TerrainCell) -> Color {
        let slant = self.world().get_slant(cell);

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

    // #[cfg(feature = "render")]
    // #[must_use]
    // fn map_colors(&self) -> Vec<Color> {
    //     self.world()
    //         .terrain
    //         .iter()
    //         .rev()
    //         .flatten()
    //         .map(|cell| self.generate_color(cell))
    //         .collect()
    // }

    #[cfg(feature = "render")]
    #[must_use]
    pub fn map_color_bytes(&self, render_settings: &WorldRenderSettings) -> Vec<u8> {
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
}
