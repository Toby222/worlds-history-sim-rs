use {
    crate::{TerrainCell, World},
    bevy::prelude::debug,
    rand::{rngs::StdRng, SeedableRng},
    serde::{
        de::{Error, MapAccess, SeqAccess, Visitor},
        Deserialize,
    },
    std::fmt::{self, Formatter},
};

struct WorldTerrainAttributes {
    max_altitude:    f32,
    min_altitude:    f32,
    max_rainfall:    f32,
    min_rainfall:    f32,
    max_temperature: f32,
    min_temperature: f32,
}
impl Default for WorldTerrainAttributes {
    fn default() -> Self {
        Self {
            max_altitude:    World::MIN_ALTITUDE,
            min_altitude:    World::MAX_ALTITUDE,
            max_rainfall:    World::MIN_RAINFALL,
            min_rainfall:    World::MAX_RAINFALL,
            max_temperature: World::MIN_TEMPERATURE,
            min_temperature: World::MAX_TEMPERATURE,
        }
    }
}

impl<'de> Deserialize<'de> for World {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "snake_case")]
        enum Field {
            Width,
            Height,
            Seed,
            Terrain,
            ContinentOffsets,
            ContinentSizes,
            Iteration,
        }

        struct WorldVisitor;

        impl<'de> Visitor<'de> for WorldVisitor {
            type Value = World;

            fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
                formatter.write_str("struct World")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let width = seq
                    .next_element()?
                    .ok_or_else(|| Error::invalid_length(0, &self))?;

                let height = seq
                    .next_element()?
                    .ok_or_else(|| Error::invalid_length(1, &self))?;

                let seed = seq
                    .next_element()?
                    .ok_or_else(|| Error::invalid_length(2, &self))?;

                let terrain: Vec<Vec<TerrainCell>> = seq
                    .next_element()?
                    .ok_or_else(|| Error::invalid_length(3, &self))?;

                let continent_offsets = seq
                    .next_element()?
                    .ok_or_else(|| Error::invalid_length(4, &self))?;

                let continent_sizes = seq
                    .next_element()?
                    .ok_or_else(|| Error::invalid_length(5, &self))?;

                debug!("Iteration aaaaa");
                let iteration = seq
                    .next_element()?
                    .ok_or_else(|| Error::invalid_length(6, &self))?;
                debug!("Iteration bbbbb");

                let world_attributes = &mut WorldTerrainAttributes::default();
                let world_attributes =
                    terrain
                        .iter()
                        .flatten()
                        .fold(world_attributes, |attributes, cell| {
                            if cell.altitude > attributes.max_altitude {
                                attributes.max_altitude = cell.altitude;
                            }
                            if cell.altitude < attributes.min_altitude {
                                attributes.min_altitude = cell.altitude;
                            }

                            if cell.rainfall > attributes.max_rainfall {
                                attributes.max_rainfall = cell.rainfall;
                            }
                            if cell.rainfall < attributes.min_rainfall {
                                attributes.min_rainfall = cell.rainfall;
                            }

                            if cell.temperature > attributes.max_temperature {
                                attributes.max_temperature = cell.temperature;
                            }
                            if cell.temperature < attributes.min_temperature {
                                attributes.min_temperature = cell.temperature;
                            }
                            attributes
                        });

                debug!("Constructing world");
                let mut world = World {
                    width,
                    height,
                    seed,
                    terrain,
                    continent_offsets,
                    continent_sizes,

                    max_altitude: world_attributes.max_altitude,
                    min_altitude: world_attributes.min_altitude,
                    max_rainfall: world_attributes.max_rainfall,
                    min_rainfall: world_attributes.min_rainfall,
                    max_temperature: world_attributes.max_temperature,
                    min_temperature: world_attributes.min_temperature,

                    rng: StdRng::seed_from_u64(seed as u64),
                    iteration,
                };
                {
                    let mut y = 0;
                    debug!("Completing terrain");
                    for terrain_row in world.terrain.iter_mut() {
                        let mut x = 0;
                        for terrain_cell in terrain_row.iter_mut() {
                            terrain_cell.x = x;
                            terrain_cell.y = y;
                            x += 1;
                        }
                        y += 1;
                    }
                }
                Ok(world)
            }

            fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut width = None;
                let mut height = None;
                let mut seed = None;
                let mut terrain = None;
                let mut continent_offsets = None;
                let mut continent_widths = None;
                let mut iteration = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Width => {
                            if width.is_some() {
                                return Err(Error::duplicate_field("width"));
                            }
                            width = Some(map.next_value()?);
                        },
                        Field::Height => {
                            if height.is_some() {
                                return Err(Error::duplicate_field("height"));
                            }
                            height = Some(map.next_value()?);
                        },
                        Field::Seed => {
                            if seed.is_some() {
                                return Err(Error::duplicate_field("seed"));
                            }
                            seed = Some(map.next_value()?);
                        },
                        Field::Terrain => {
                            if terrain.is_some() {
                                return Err(Error::duplicate_field("terrain"));
                            }
                            terrain = Some(map.next_value()?);
                        },
                        Field::ContinentOffsets => {
                            if continent_offsets.is_some() {
                                return Err(Error::duplicate_field("continent_offsets"));
                            }
                            continent_offsets = Some(map.next_value()?);
                        },
                        Field::ContinentSizes => {
                            if continent_widths.is_some() {
                                return Err(Error::duplicate_field("continent_widths"));
                            }
                            continent_widths = Some(map.next_value()?);
                        },
                        Field::Iteration => {
                            if iteration.is_some() {
                                return Err(Error::duplicate_field("iteration"));
                            }
                            iteration = Some(map.next_value()?);
                        },
                    }
                }

                let width = width.ok_or_else(|| Error::missing_field("width"))?;
                let height = height.ok_or_else(|| Error::missing_field("height"))?;
                let seed = seed.ok_or_else(|| Error::missing_field("seed"))?;
                let mut terrain: Vec<Vec<TerrainCell>> =
                    terrain.ok_or_else(|| Error::missing_field("terrain"))?;

                for x in 0..width as usize {
                    for y in 0..height as usize {
                        terrain[y][x].x = x;
                        terrain[y][x].y = y;
                    }
                }

                let continent_offsets =
                    continent_offsets.ok_or_else(|| Error::missing_field("continent_offsets"))?;
                let continent_widths =
                    continent_widths.ok_or_else(|| Error::missing_field("continent_widths"))?;

                let world_attributes = &mut WorldTerrainAttributes::default();
                let world_attributes =
                    terrain
                        .iter()
                        .flatten()
                        .fold(world_attributes, |attributes, cell| {
                            if cell.altitude > attributes.max_altitude {
                                attributes.max_altitude = cell.altitude;
                            }
                            if cell.altitude < attributes.min_altitude {
                                attributes.min_altitude = cell.altitude;
                            }

                            if cell.rainfall > attributes.max_rainfall {
                                attributes.max_rainfall = cell.rainfall;
                            }
                            if cell.rainfall < attributes.min_rainfall {
                                attributes.min_rainfall = cell.rainfall;
                            }

                            if cell.temperature > attributes.max_temperature {
                                attributes.max_temperature = cell.temperature;
                            }
                            if cell.temperature < attributes.min_temperature {
                                attributes.min_temperature = cell.temperature;
                            }
                            attributes
                        });

                let iteration = iteration.ok_or_else(|| Error::missing_field("iteration"))?;

                let mut world = World {
                    width,
                    height,
                    seed,
                    terrain,
                    continent_offsets,
                    continent_sizes: continent_widths,

                    max_altitude: world_attributes.max_altitude,
                    min_altitude: world_attributes.min_altitude,
                    max_rainfall: world_attributes.max_rainfall,
                    min_rainfall: world_attributes.min_rainfall,
                    max_temperature: world_attributes.max_temperature,
                    min_temperature: world_attributes.min_temperature,

                    rng: StdRng::seed_from_u64(seed as u64),
                    iteration,
                };
                {
                    let mut y = 0;
                    for terrain_row in world.terrain.iter_mut() {
                        let mut x = 0;
                        for terrain_cell in terrain_row.iter_mut() {
                            terrain_cell.x = x;
                            terrain_cell.y = y;
                            x += 1;
                        }
                        y += 1;
                    }
                }
                Ok(world)
            }
        }

        const FIELDS: &'static [&'static str] = &[
            "width",
            "height",
            "seed",
            "terrain",
            "continent_offsets",
            "continent_widths",
        ];

        deserializer.deserialize_struct("World", FIELDS, WorldVisitor)
    }
}
