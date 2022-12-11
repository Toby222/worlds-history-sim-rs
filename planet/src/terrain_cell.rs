use {
    crate::{perlin, BiomeType, HumanGroup, World},
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TerrainCell {
    pub altitude:    f32,
    pub rainfall:    f32,
    pub temperature: f32,

    #[serde(skip)]
    pub x:               usize,
    #[serde(skip)]
    pub y:               usize,
    pub local_iteration: usize,

    pub biome_presences: Vec<(BiomeType, f32)>,
    pub human_groups:    Vec<HumanGroup>,
}

impl TerrainCell {
    pub fn get_next_local_random_int(&mut self, world: &World) -> u32 {
        let seed = world.seed;

        let x = seed as f32 + self.x as f32;
        let y = seed as f32 + self.y as f32;
        let z = seed as f32 + world.iteration as f32 + (self.local_iteration - 1) as f32;

        self.local_iteration += 1;

        perlin::permutation_value(x, y, z)
    }

    pub fn get_next_local_random_float(&mut self, world: &World) -> f32 {
        self.get_next_local_random_int(world) as f32 / perlin::MAX_PERMUTATION_VALUE as f32
    }

    pub fn biome_presence(&self, biome: BiomeType) -> f32 {
        if let Some(presence) = self
            .biome_presences
            .iter()
            .find(|biome_presence| biome_presence.0 == biome)
        {
            presence.1
        } else {
            0.0
        }
    }
}