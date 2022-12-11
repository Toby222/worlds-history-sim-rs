use {
    crate::TerrainCell,
    bevy::prelude::default,
    serde::{
        de::{Error, MapAccess, SeqAccess, Visitor},
        Deserialize,
    },
    std::{
        fmt::{self, Formatter},
        sync::Weak,
    },
};

impl<'de> Deserialize<'de> for TerrainCell {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "snake_case")]
        enum Field {
            Altitude,
            Rainfall,
            Temperature,
            LocalIteration,
            BiomePresences,
        }

        struct TerrainCellVisitor;

        impl<'de> Visitor<'de> for TerrainCellVisitor {
            type Value = TerrainCell;

            fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
                formatter.write_str("struct TerrainCell")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let altitude = seq
                    .next_element()?
                    .ok_or_else(|| Error::invalid_length(0, &self))?;

                let rainfall = seq
                    .next_element()?
                    .ok_or_else(|| Error::invalid_length(1, &self))?;

                let temperature = seq
                    .next_element()?
                    .ok_or_else(|| Error::invalid_length(2, &self))?;

                let local_iteration = seq
                    .next_element()?
                    .ok_or_else(|| Error::invalid_length(3, &self))?;

                let biome_presences = seq
                    .next_element()?
                    .ok_or_else(|| Error::invalid_length(4, &self))?;

                Ok(TerrainCell {
                    altitude,
                    rainfall,
                    temperature,
                    local_iteration,
                    biome_presences,
                    x: default(),
                    y: default(),
                })
            }

            fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut altitude = None;
                let mut rainfall = None;
                let mut temperature = None;
                let mut local_iteration = None;
                let mut biome_presences = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Altitude => {
                            if altitude.is_some() {
                                return Err(Error::duplicate_field("altitude"));
                            }
                            altitude = Some(map.next_value()?);
                        },
                        Field::Rainfall => {
                            if rainfall.is_some() {
                                return Err(Error::duplicate_field("rainfall"));
                            }
                            rainfall = Some(map.next_value()?);
                        },
                        Field::Temperature => {
                            if temperature.is_some() {
                                return Err(Error::duplicate_field("temperature"));
                            }
                            temperature = Some(map.next_value()?);
                        },
                        Field::LocalIteration => {
                            if local_iteration.is_some() {
                                return Err(Error::duplicate_field("local_iteration"));
                            }
                            local_iteration = Some(map.next_value()?);
                        },
                        Field::BiomePresences => {
                            if biome_presences.is_some() {
                                return Err(Error::duplicate_field("biome_presences"));
                            }
                            biome_presences = Some(map.next_value()?);
                        },
                    }
                }

                let altitude = altitude.ok_or_else(|| Error::missing_field("altitude"))?;
                let rainfall = rainfall.ok_or_else(|| Error::missing_field("rainfall"))?;
                let temperature = temperature.ok_or_else(|| Error::missing_field("temperature"))?;
                let local_iteration =
                    local_iteration.ok_or_else(|| Error::missing_field("local_iteration"))?;
                let biome_presences =
                    biome_presences.ok_or_else(|| Error::missing_field("biome_presences"))?;

                Ok(TerrainCell {
                    altitude,
                    rainfall,
                    temperature,
                    local_iteration,
                    biome_presences,
                    x: default(),
                    y: default(),
                })
            }
        }

        const FIELDS: &'static [&'static str] = &[
            "altitude",
            "rainfall",
            "temperature",
            "local_iteration",
            "biome_presences",
        ];

        deserializer.deserialize_struct("TerrainCell", FIELDS, TerrainCellVisitor)
    }
}
