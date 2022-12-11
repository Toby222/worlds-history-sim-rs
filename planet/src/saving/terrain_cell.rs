use {
    crate::{HumanGroup, TerrainCell},
    bevy::prelude::{debug, default},
    serde::{
        de::{Error, MapAccess, SeqAccess, Visitor},
        ser::SerializeStruct,
        Deserialize,
        Serialize,
    },
    std::{
        fmt::{self, Formatter},
        sync::Arc,
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
            Height,
            Width,
            HumanGroups,
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
                std::mem::transmute::<u128>(seq);

                let mut length = 0;
                let altitude = seq
                    .next_element()?
                    .ok_or_else(|| panic!("Invalid length {length}, expected 8"))?;
                length += 1;

                let rainfall = seq
                    .next_element()?
                    .ok_or_else(|| panic!("Invalid length {length}, expected 8"))?;
                length += 1;

                let temperature = seq
                    .next_element()?
                    .ok_or_else(|| panic!("Invalid length {length}, expected 8"))?;
                length += 1;

                let local_iteration = seq
                    .next_element()?
                    .ok_or_else(|| panic!("Invalid length {length}, expected 8"))?;
                length += 1;

                let biome_presences = seq
                    .next_element()?
                    .ok_or_else(|| panic!("Invalid length {length}, expected 8"))?;
                length += 1;

                let height = seq
                    .next_element()?
                    .ok_or_else(|| panic!("Invalid length {length}, expected 8"))?;
                length += 1;

                let width = seq
                    .next_element()?
                    .ok_or_else(|| panic!("Invalid length {length}, expected 8"))?;
                length += 1;

                let human_groups = seq
                    .next_element::<Vec<HumanGroup>>()?
                    .ok_or_else(|| panic!("Invalid length {length}, expected 8"))?;
                // length += 1;

                Ok(TerrainCell {
                    altitude,
                    rainfall,
                    temperature,
                    local_iteration,
                    biome_presences,
                    x: default(),
                    y: default(),
                    human_groups: human_groups.iter().map(|group| Arc::new(*group)).collect(),
                    height,
                    width,
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
                let mut height = None;
                let mut width = None;
                let mut human_groups: Option<Vec<HumanGroup>> = None;

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
                        Field::Height => {
                            if height.is_some() {
                                return Err(Error::duplicate_field("height"));
                            }
                            height = Some(map.next_value()?);
                        },
                        Field::Width => {
                            if width.is_some() {
                                return Err(Error::duplicate_field("width"));
                            }
                            width = Some(map.next_value()?);
                        },
                        Field::HumanGroups => {
                            if human_groups.is_some() {
                                return Err(Error::duplicate_field("human_groups"));
                            }
                            human_groups = Some(map.next_value()?);
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
                let height = height.ok_or_else(|| Error::missing_field("height"))?;
                let width = width.ok_or_else(|| Error::missing_field("width"))?;
                let human_groups =
                    human_groups.ok_or_else(|| Error::missing_field("human_groups"))?;

                Ok(TerrainCell {
                    altitude,
                    rainfall,
                    temperature,
                    local_iteration,
                    biome_presences,
                    x: default(),
                    y: default(),
                    human_groups: human_groups.iter().map(|group| Arc::new(*group)).collect(),
                    height,
                    width,
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

impl Serialize for TerrainCell {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let TerrainCell {
            altitude,
            rainfall,
            temperature,
            x: _x,
            y: _y,
            local_iteration,
            biome_presences,
            human_groups,
            height,
            width,
        } = self;

        let human_groups = &human_groups
            .iter()
            .map(|group_arc| **group_arc)
            .collect::<Vec<_>>();

        let mut serialized_struct = serializer.serialize_struct(stringify!(TerrainCell), 10)?;
        serialized_struct.serialize_field(stringify!(altitude), altitude)?;
        serialized_struct.serialize_field(stringify!(rainfall), rainfall)?;
        serialized_struct.serialize_field(stringify!(temperature), temperature)?;
        // #[skip]
        // serialized_struct.serialize_field(stringify!(x), x)?;
        // #[skip]
        // serialized_struct.serialize_field(stringify!(y), y)?;
        serialized_struct.serialize_field(stringify!(local_iteration), local_iteration)?;
        serialized_struct.serialize_field(stringify!(biome_presences), biome_presences)?;
        serialized_struct.serialize_field(stringify!(human_groups), human_groups)?;
        serialized_struct.serialize_field(stringify!(height), height)?;
        serialized_struct.serialize_field(stringify!(width), width)?;
        serialized_struct.end()
    }
}
