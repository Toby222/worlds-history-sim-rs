#[cfg(all(feature = "logging", feature = "render"))]
use bevy::log::debug;
#[cfg(feature = "logging")]
use bevy::utils::default;
#[cfg(all(feature = "render", feature = "globe_view"))]
use std::f32::consts::PI;
use {
    crate::{macros::iterable_enum, World, WorldGenError},
    bevy::log::warn,
    rand::random,
    std::{
        error::Error,
        fmt::Display,
        fs::File,
        io::{self, Read, Write},
        path::Path,
    },
};
#[cfg(feature = "render")]
use {
    crate::{BiomeStats, TerrainCell},
    bevy::{
        asset::{Assets, HandleId},
        render::render_resource::Extent3d,
        render::{color::Color, texture::Image},
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

iterable_enum!(PlanetView { Biomes, Altitude });

#[cfg(feature = "render")]
#[derive(Debug, Default)]
pub struct WorldRenderSettings {
    pub map_image_handle_id:      Option<HandleId>,
    #[cfg(feature = "globe_view")]
    pub globe_image_handle_id:    Option<HandleId>,
    #[cfg(feature = "globe_view")]
    pub globe_material_handle_id: Option<HandleId>,

    rainfall_visible:    bool,
    temperature_visible: bool,
    view:                PlanetView,
}

#[cfg(feature = "render")]
impl WorldRenderSettings {
    #[cfg(feature = "render")]
    pub fn toggle_rainfall(&mut self) {
        #[cfg(feature = "logging")]
        if self.rainfall_visible {
            debug!("Turning rainfall off");
        } else {
            debug!("Turning rainfall on");
        }
        self.rainfall_visible = !self.rainfall_visible;
    }

    #[cfg(feature = "render")]
    pub fn toggle_temperature(&mut self) {
        #[cfg(feature = "logging")]
        if self.temperature_visible {
            debug!("Turning temperature off");
        } else {
            debug!("Turning temperature on");
        }
        self.temperature_visible = !self.temperature_visible;
    }

    #[cfg(feature = "render")]
    pub fn cycle_view(&mut self) {
        let idx = (PlanetView::iterator()
            .position(|view| *view == self.view)
            .unwrap()
            + 1)
            % PlanetView::ITEM_COUNT;
        #[cfg(feature = "logging")]
        debug!(
            "Cycling view from {:#?} to {:#?}",
            self.view,
            PlanetView::ITEMS[idx]
        );
        self.view = PlanetView::ITEMS[idx];
    }
}

impl Default for PlanetView {
    fn default() -> Self {
        PlanetView::Biomes
    }
}

#[derive(Debug, Default)]
pub struct WorldManager {
    world: Option<World>,

    #[cfg(feature = "render")]
    pub render_settings: WorldRenderSettings,
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

    pub fn load_world<P: AsRef<Path>>(
        &mut self,
        path: P,
        #[cfg(feature = "render")] images: &mut Assets<Image>,
    ) -> Result<(), LoadError> {
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
                #[cfg(feature = "render")]
                let World { height, width, .. } = world;
                self.world = Some(world);
                #[cfg(feature = "render")]
                {
                    let image_handle = &images.get_handle(
                        self.render_settings
                            .map_image_handle_id
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
    fn generate_color(&self, cell: &TerrainCell) -> Color {
        if self.render_settings.view == PlanetView::Biomes {
            return WorldManager::biome_color(cell);
        }

        let altitude_color = WorldManager::altitude_contour_color(cell.altitude);
        // let altitude_color = if self.contours {
        //     WorldManager::altitude_contour_color(cell.altitude)
        // } else {
        //     WorldManager::altitude_color(cell.altitude)
        // };

        let mut layer_count = 1.0;

        let mut red = altitude_color.r();
        let mut green = altitude_color.g();
        let mut blue = altitude_color.b();

        if self.render_settings.rainfall_visible {
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

        if self.render_settings.temperature_visible {
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
    fn biome_color(cell: &TerrainCell) -> Color {
        cell.biome_presences
            .iter()
            .fold(Color::BLACK, |color, (biome_type, presence)| {
                let biome: BiomeStats = (*biome_type).into();
                let biome_color = biome.color;

                Color::rgb(
                    color.r() + (biome_color.r() * presence),
                    color.g() + (biome_color.g() * presence),
                    color.b() + (biome_color.b() * presence),
                )
            })
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
    pub fn map_color_bytes(&self) -> Vec<u8> {
        self.world()
            .terrain
            .iter()
            .rev()
            .flatten()
            .flat_map(|cell| {
                self.generate_color(cell)
                    .as_rgba_f32()
                    .iter()
                    .flat_map(|num| num.to_le_bytes())
                    .collect::<Vec<u8>>()
            })
            .collect()
    }

    #[cfg(all(feature = "render", feature = "globe_view"))]
    #[must_use]
    fn globe_colors(&self) -> Vec<Color> {
        let world = self.world();
        let width = world.width as usize;
        let height = world.height as usize;

        let mut colors = vec![Color::PINK; height * width];

        for y in 0..world.height as usize * 2 {
            for x in 0..world.width as usize {
                let factor_y = (1.0 - f32::cos(PI * y as f32 / (world.height * 2) as f32)) / 2.0;
                let real_y = f32::floor(world.height as f32 * factor_y) as usize;
                #[cfg(feature = "logging")]
                assert!(
                    real_y < world.height as usize,
                    "Trying to get cell off of planet. {}/{}",
                    real_y,
                    world.height
                );

                colors[real_y * width + x] = self.generate_color(&world.terrain[real_y][x]);
            }
        }
        colors
    }

    #[cfg(all(feature = "render", feature = "globe_view"))]
    #[must_use]
    pub fn globe_color_bytes(&self) -> Vec<u8> {
        self.globe_colors()
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
