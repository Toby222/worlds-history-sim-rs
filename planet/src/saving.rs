use std::fmt::{self, Formatter};

use rand::{rngs::StdRng, SeedableRng};
use serde::{
    de::{Error, MapAccess, SeqAccess, Visitor},
    Deserialize,
};

use crate::{TerrainCell, World};

struct WorldTerrainAttributes {
    max_altitude: f32,
    min_altitude: f32,
    max_rainfall: f32,
    min_rainfall: f32,
    max_temperature: f32,
    min_temperature: f32,
}
impl Default for WorldTerrainAttributes {
    fn default() -> Self {
        Self {
            max_altitude: World::MIN_ALTITUDE,
            min_altitude: World::MAX_ALTITUDE,
            max_rainfall: World::MIN_RAINFALL,
            min_rainfall: World::MAX_RAINFALL,
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
            ContinentWidths,
        }

        struct WorldVisitor;

        impl<'de> Visitor<'de> for WorldVisitor {
            type Value = World;

            fn expecting(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
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
                    .ok_or_else(|| Error::invalid_length(0, &self))?;

                let seed = seq
                    .next_element()?
                    .ok_or_else(|| Error::invalid_length(0, &self))?;

                let terrain: Vec<Vec<TerrainCell>> = seq
                    .next_element()?
                    .ok_or_else(|| Error::invalid_length(0, &self))?;

                let continent_offsets = seq
                    .next_element()?
                    .ok_or_else(|| Error::invalid_length(0, &self))?;

                let continent_widths = seq
                    .next_element()?
                    .ok_or_else(|| Error::invalid_length(0, &self))?;

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

                Ok(World {
                    width,
                    height,
                    seed,
                    terrain,
                    continent_offsets,
                    continent_widths,

                    max_altitude: world_attributes.max_altitude,
                    min_altitude: world_attributes.min_altitude,
                    max_rainfall: world_attributes.max_rainfall,
                    min_rainfall: world_attributes.min_rainfall,
                    max_temperature: world_attributes.max_temperature,
                    min_temperature: world_attributes.min_temperature,

                    rng: StdRng::seed_from_u64(seed as u64),
                })
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

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Width => {
                            if width.is_some() {
                                return Err(Error::duplicate_field("width"));
                            }
                            width = Some(map.next_value()?);
                        }
                        Field::Height => {
                            if height.is_some() {
                                return Err(Error::duplicate_field("height"));
                            }
                            height = Some(map.next_value()?);
                        }
                        Field::Seed => {
                            if seed.is_some() {
                                return Err(Error::duplicate_field("seed"));
                            }
                            seed = Some(map.next_value()?);
                        }
                        Field::Terrain => {
                            if terrain.is_some() {
                                return Err(Error::duplicate_field("terrain"));
                            }
                            terrain = Some(map.next_value()?);
                        }
                        Field::ContinentOffsets => {
                            if continent_offsets.is_some() {
                                return Err(Error::duplicate_field("continent_offsets"));
                            }
                            continent_offsets = Some(map.next_value()?);
                        }
                        Field::ContinentWidths => {
                            if continent_widths.is_some() {
                                return Err(Error::duplicate_field("continent_widths"));
                            }
                            continent_widths = Some(map.next_value()?);
                        }
                    }
                }

                let width = width.ok_or_else(|| Error::missing_field("width"))?;
                let height = height.ok_or_else(|| Error::missing_field("height"))?;
                let seed = seed.ok_or_else(|| Error::missing_field("seed"))?;
                let terrain: Vec<Vec<TerrainCell>> =
                    terrain.ok_or_else(|| Error::missing_field("terrain"))?;
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

                Ok(World {
                    width,
                    height,
                    seed,
                    terrain,
                    continent_offsets,
                    continent_widths,

                    max_altitude: world_attributes.max_altitude,
                    min_altitude: world_attributes.min_altitude,
                    max_rainfall: world_attributes.max_rainfall,
                    min_rainfall: world_attributes.min_rainfall,
                    max_temperature: world_attributes.max_temperature,
                    min_temperature: world_attributes.min_temperature,

                    rng: StdRng::seed_from_u64(seed as u64),
                })
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
