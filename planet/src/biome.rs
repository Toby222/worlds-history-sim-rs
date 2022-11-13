use {
    crate::{macros::iterable_enum, World},
    bevy::render::color::Color,
};

#[derive(Debug, Clone, Default)]
pub struct BiomeStats {
    pub name:            String,
    pub color:           Color,
    pub min_altitude:    f32,
    pub max_altitude:    f32,
    pub min_rainfall:    f32,
    pub max_rainfall:    f32,
    pub min_temperature: f32,
    pub max_temperature: f32,
}

iterable_enum!(BiomeType {
    IceCap,
    Ocean,
    Grassland,
    Forest,
    Taiga,
    Tundra,
    Desert,
    Rainforest
});

impl From<BiomeType> for BiomeStats {
    fn from(biome_type: BiomeType) -> BiomeStats {
        match biome_type {
            BiomeType::IceCap => BiomeStats {
                name:            "Ice Cap".into(),
                color:           Color::rgb_u8(253, 244, 235),
                min_altitude:    World::MIN_ALTITUDE,
                max_altitude:    World::MAX_ALTITUDE,
                min_rainfall:    World::MIN_RAINFALL,
                max_rainfall:    World::MAX_RAINFALL,
                min_temperature: World::MIN_TEMPERATURE,
                max_temperature: -15.0,
            },
            BiomeType::Ocean => BiomeStats {
                name:            "Ocean".into(),
                color:           Color::rgb_u8(28, 66, 84),
                min_altitude:    World::MIN_ALTITUDE,
                max_altitude:    0.0,
                min_rainfall:    World::MIN_RAINFALL,
                max_rainfall:    World::MAX_RAINFALL,
                min_temperature: -15.0,
                max_temperature: World::MAX_TEMPERATURE,
            },
            BiomeType::Grassland => BiomeStats {
                name:            "Grassland".into(),
                color:           Color::rgb_u8(167, 177, 84),
                min_altitude:    0.0,
                max_altitude:    World::MAX_ALTITUDE,
                min_rainfall:    25.0,
                max_rainfall:    1475.0,
                min_temperature: -5.0,
                max_temperature: World::MAX_TEMPERATURE,
            },
            BiomeType::Forest => BiomeStats {
                name:            "Forest".into(),
                color:           Color::rgb_u8(76, 132, 55),
                min_altitude:    0.0,
                max_altitude:    World::MAX_ALTITUDE,
                min_rainfall:    975.0,
                max_rainfall:    2475.0,
                min_temperature: -5.0,
                max_temperature: World::MAX_TEMPERATURE,
            },
            BiomeType::Taiga => BiomeStats {
                name:            "Taiga".into(),
                color:           Color::rgb_u8(43, 63, 40),
                min_altitude:    0.0,
                max_altitude:    World::MAX_ALTITUDE,
                min_rainfall:    475.0,
                max_rainfall:    World::MAX_RAINFALL,
                min_temperature: -15.0,
                max_temperature: -0.0,
            },
            BiomeType::Tundra => BiomeStats {
                name:            "Tundra ".into(),
                color:           Color::rgb_u8(139, 139, 128),
                min_altitude:    0.0,
                max_altitude:    World::MAX_ALTITUDE,
                min_rainfall:    World::MIN_RAINFALL,
                max_rainfall:    725.0,
                min_temperature: -20.0,
                max_temperature: -0.0,
            },
            BiomeType::Desert => BiomeStats {
                name:            "Desert ".into(),
                color:           Color::rgb_u8(253, 225, 171),
                min_altitude:    0.0,
                max_altitude:    World::MAX_ALTITUDE,
                min_rainfall:    World::MIN_RAINFALL,
                max_rainfall:    125.0,
                min_temperature: -5.0,
                max_temperature: World::MAX_TEMPERATURE,
            },
            BiomeType::Rainforest => BiomeStats {
                name:            "Rainforest".into(),
                color:           Color::rgb_u8(59, 103, 43),
                min_altitude:    0.0,
                max_altitude:    World::MAX_ALTITUDE,
                min_rainfall:    1975.0,
                max_rainfall:    World::MAX_RAINFALL,
                min_temperature: -5.0,
                max_temperature: World::MAX_TEMPERATURE,
            },
        }
    }
}

impl From<&BiomeType> for BiomeStats {
    fn from(biome_type: &BiomeType) -> BiomeStats {
        (*biome_type).into()
    }
}
