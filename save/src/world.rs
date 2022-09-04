use std::{
    error::Error,
    f32::consts::{PI, TAU},
    fmt::Display,
};

use bevy::{math::Vec3A, prelude::Vec2, utils::default};
use noise::{NoiseFn, Perlin, Seedable};
use rand::Rng;

use crate::{cartesian_coordinates, mix_values, random_point_in_sphere, CartesianError, RepeatNum};

#[derive(Debug, Clone, Copy)]
pub enum WorldGenError {
    CartesianError(CartesianError),
}
impl Error for WorldGenError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            WorldGenError::CartesianError(ref e) => Some(e),
        }
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}
impl Display for WorldGenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WorldGenError::CartesianError(err) => err.fmt(f),
        }
    }
}

#[derive(Debug, Clone)]
pub struct World {
    pub width: i32,
    pub height: i32,
    pub seed: u32,

    pub terrain: Vec<Vec<TerrainCell>>,
    contintent_offsets: [Vec2; World::NUM_CONTINENTS as usize],
    perlin: Perlin,
}

#[derive(Debug, Copy, Clone, Default)]
pub struct TerrainCell {
    pub altitude: f32,
    pub rainfall: f32,
}

impl World {
    pub fn new(width: i32, height: i32, seed: u32) -> World {
        World {
            width,
            height,
            seed,
            terrain: vec![
                vec![TerrainCell::default(); width.try_into().unwrap()];
                height.try_into().unwrap()
            ],
            contintent_offsets: [default(); Self::NUM_CONTINENTS as usize],
            perlin: Perlin::new().set_seed(seed),
        }
    }

    pub const NUM_CONTINENTS: u8 = 3;
    pub const CONTINENT_FACTOR: f32 = 0.7;
    pub const CONTINENT_WIDTH_FACTOR: f32 = 5.0;

    pub const MIN_ALTITUDE: f32 = -10000.0;
    pub const MAX_ALTITUDE: f32 = 10000.0;
    pub const ALTITUDE_SPAN: f32 = Self::MAX_ALTITUDE - Self::MIN_ALTITUDE;

    pub const MOUNTAIN_RANGE_WIDTH_FACTOR: f32 = 15.0;

    pub const TERRAIN_NOISE_FACTOR_1: f32 = 0.2;
    pub const TERRAIN_NOISE_FACTOR_2: f32 = 0.15;
    pub const TERRAIN_NOISE_FACTOR_3: f32 = 0.1;

    pub const MIN_RAINFALL: f32 = -10.0;
    pub const MAX_RAINFALL: f32 = 100.0;
    pub const RAINFALL_SPAN: f32 = Self::MAX_RAINFALL - Self::MIN_RAINFALL;
    pub const RAINFALL_ALTITUDE_FACTOR: f32 = 1.0;

    pub fn generate(&mut self) -> Result<(), WorldGenError> {
        if let Err(err) = self.generate_altitude() {
            return Err(WorldGenError::CartesianError(err));
        }
        if let Err(err) = self.generate_rainfall() {
            return Err(WorldGenError::CartesianError(err));
        }

        Ok(())
    }

    fn generate_altitude(&mut self) -> Result<(), CartesianError> {
        self.generate_continents();

        let offset_1 = Self::random_offset_vector();
        let offset_2 = Self::random_offset_vector();
        let offset_3 = Self::random_offset_vector();
        let offset_4 = Self::random_offset_vector();
        let offset_5 = Self::random_offset_vector();

        const RADIUS_1: f32 = 0.5;
        const RADIUS_2: f32 = 4.0;
        const RADIUS_3: f32 = 4.0;
        const RADIUS_4: f32 = 8.0;
        const RADIUS_5: f32 = 16.0;

        for y in 0..self.terrain.len() {
            let alpha = (y as f32 / self.height as f32) * PI;

            for x in 0..self.terrain[y].len() {
                let beta = (x as f32 / self.width as f32) * TAU;

                let continent_value = self.continent_modifier(x, y);

                let value_1 =
                    self.random_noise_from_polar_coordinates(alpha, beta, RADIUS_1, offset_1)?;
                let value_2 =
                    self.random_noise_from_polar_coordinates(alpha, beta, RADIUS_2, offset_2)?;
                let value_3 =
                    self.random_noise_from_polar_coordinates(alpha, beta, RADIUS_3, offset_3)?;
                let value_4 =
                    self.random_noise_from_polar_coordinates(alpha, beta, RADIUS_4, offset_4)?;
                let value_5 =
                    self.random_noise_from_polar_coordinates(alpha, beta, RADIUS_5, offset_5)?;

                let mut raw_altitude = self
                    .random_mountain_noise_from_random_noise(mix_values(value_1, value_2, 0.1))
                    * mix_values(1.0, continent_value, 0.3);

                raw_altitude = mix_values(raw_altitude, continent_value, Self::CONTINENT_FACTOR);
                raw_altitude = mix_values(raw_altitude, value_3, Self::TERRAIN_NOISE_FACTOR_1);
                raw_altitude *= mix_values(1.0, value_4, Self::TERRAIN_NOISE_FACTOR_2);
                raw_altitude *= mix_values(1.0, value_5, Self::TERRAIN_NOISE_FACTOR_3);

                self.terrain[y][x].altitude = Self::calculate_altitude(raw_altitude);
            }
        }
        Ok(())
    }

    fn calculate_altitude(raw_altitude: f32) -> f32 {
        Self::MIN_ALTITUDE + (raw_altitude * Self::ALTITUDE_SPAN)
    }

    fn generate_rainfall(&mut self) -> Result<(), CartesianError> {
        let offset = Self::random_offset_vector();
        const RADIUS: f32 = 2.0;

        for (y, row) in self.terrain.iter_mut().enumerate() {
            let alpha = (y as f32 / self.height as f32) * PI;
            for (x, cell) in row.iter_mut().enumerate() {
                let beta = (x as f32 / self.width as f32) * TAU;

                let pos = cartesian_coordinates(alpha, beta, RADIUS)? + offset;

                let value = self.perlin.get([pos.x.into(), pos.y.into(), pos.z.into()]) as f32;

                let base_rainfall = (value * Self::RAINFALL_SPAN + Self::MIN_RAINFALL)
                    .clamp(0.0, World::MAX_RAINFALL);
                let altitude_factor = ((cell.altitude / Self::MAX_ALTITUDE)
                    * World::RAINFALL_ALTITUDE_FACTOR)
                    .clamp(0.0, 1.0);
                let rainfall = base_rainfall * (1.0 - altitude_factor);

                cell.rainfall = rainfall;
            }
        }
        Ok(())
    }

    fn generate_continents(&mut self) {
        let mut rng = rand::thread_rng();
        for (idx, continent_offset) in self.contintent_offsets.iter_mut().enumerate() {
            continent_offset.x = rng
                .gen_range(
                    self.width as f32 * idx as f32 * 2.0 / 5.0
                        ..self.width as f32 * (idx as f32 + 2.0) * 2.0 / 5.0,
                )
                .repeat(self.width as f32);
            continent_offset.y =
                rng.gen_range(self.height as f32 * 2.0 / 7.0..self.height as f32 * 5.0 / 7.0);
        }
    }

    fn continent_modifier(&self, x: usize, y: usize) -> f32 {
        let x = x as f32;
        let y = y as f32;
        let width = self.width as f32;
        let height = self.height as f32;

        let mut max_value = 0.0;
        let beta_factor =
            Self::CONTINENT_WIDTH_FACTOR * width / 1.5 * (1.0 - f32::sin(PI * y / height));

        for Vec2 {
            x: continent_x,
            y: contintent_y,
        } in self.contintent_offsets
        {
            let distance_x = f32::min(
                f32::min(f32::abs(continent_x - x), f32::abs(width + continent_x - x)),
                continent_x - x - width,
            );
            let distance_y = 2.0 * f32::abs(contintent_y - y);

            let distance = (distance_x * distance_x + distance_y * distance_y).sqrt();

            max_value = f32::max(
                max_value,
                f32::max(
                    0.0,
                    1.0 - Self::CONTINENT_WIDTH_FACTOR * distance / (width + beta_factor),
                ),
            );
        }

        max_value
    }

    fn random_offset_vector() -> Vec3A {
        random_point_in_sphere(1000.0)
    }

    fn random_mountain_noise_from_random_noise(&self, noise: f32) -> f32 {
        let noise = noise * 2.0 - 1.0;

        let value_1 = -(-(noise * Self::MOUNTAIN_RANGE_WIDTH_FACTOR + 1.0).powf(2.0)).exp();
        let value_2 = (-(noise * Self::MOUNTAIN_RANGE_WIDTH_FACTOR - 1.0).powf(2.0)).exp();

        (value_1 + value_2 + 1.0) / 2.0
    }

    fn random_noise_from_polar_coordinates(
        &self,
        alpha: f32,
        beta: f32,
        radius: f32,
        offset: Vec3A,
    ) -> Result<f32, CartesianError> {
        let offset = cartesian_coordinates(alpha, beta, radius)? + offset;
        Ok(self
            .perlin
            .get([offset.x as f64, offset.y as f64, offset.z as f64]) as f32)
    }
}
