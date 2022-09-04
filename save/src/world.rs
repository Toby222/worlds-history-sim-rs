use std::{
    error::Error,
    f32::consts::{PI, TAU},
    fmt::Display,
};

use bevy::{math::Vec3A, prelude::Vec2, utils::default};
use noise::{NoiseFn, Perlin, Seedable};
use rand::Rng;

use crate::{cartesian_coordinates, mix_values, random_point_in_sphere, CartesianError};

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
    pub const CONTINTENT_FACTOR: f32 = 0.5;

    pub const MIN_ALTITUDE: f32 = -10000.0;
    pub const MAX_ALTITUDE: f32 = 10000.0;
    pub const ALTITUDE_SPAN: f32 = Self::MAX_ALTITUDE - Self::MIN_ALTITUDE;

    pub const MOUNTAIN_RANGE_FACTOR: f32 = 0.1;
    pub const MOUNTAIN_RANGE_WIDTH_FACTOR: f32 = 10.0;

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
        const RADIUS_1: f32 = 2.0;

        let offset_2 = Self::random_offset_vector();
        const RADIUS_2: f32 = 1.0;

        for y in 0..self.terrain.len() {
            let alpha = (y as f32 / self.height as f32) * PI;

            for x in 0..self.terrain[y].len() {
                let beta = (x as f32 / self.width as f32) * TAU;

                let value_1 =
                    self.random_noise_from_polar_coordinates(alpha, beta, RADIUS_1, offset_1)?;
                let value_2 = self.random_mountain_noise_from_polar_coordinates(
                    alpha, beta, RADIUS_2, offset_2,
                )?;

                let raw_altitude = mix_values(value_1, value_2, Self::MOUNTAIN_RANGE_FACTOR);
                let raw_altitude = mix_values(
                    raw_altitude,
                    self.get_continent_modifier(x, y),
                    Self::CONTINTENT_FACTOR,
                );

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
        self.contintent_offsets.fill_with(|| Vec2 {
            x: rng.gen_range(1.0..(self.width - 1) as f32),
            y: rng.gen_range(1.0..(self.width - 1) as f32),
        });
    }

    fn get_continent_modifier(&self, x: usize, y: usize) -> f32 {
        let mut max_value = 0.0;

        for Vec2 {
            x: cont_x,
            y: cont_y,
        } in self.contintent_offsets
        {
            let distance_x = f32::min(
                f32::abs(cont_x - x as f32),
                f32::abs(self.width as f32 + cont_x - x as f32),
            );
            let distance_y = f32::abs(cont_y - y as f32);

            let factor_x = f32::max(0.0, 1.0 - distance_x / self.width as f32);
            let factor_y = f32::max(0.0, 1.0 - distance_y / self.height as f32);

            max_value = f32::max(max_value, factor_x * factor_x * factor_y * factor_y);
        }

        max_value
    }

    fn random_offset_vector() -> Vec3A {
        random_point_in_sphere(1000.0)
    }

    fn random_mountain_noise_from_polar_coordinates(
        &self,
        alpha: f32,
        beta: f32,
        radius: f32,
        offset: Vec3A,
    ) -> Result<f32, CartesianError> {
        let noise = World::random_noise_from_polar_coordinates(self, alpha, beta, radius, offset)?
            * 2.0
            - 1.0;

        let value_1 = (-(noise * Self::MOUNTAIN_RANGE_WIDTH_FACTOR + 1.0).powf(2.0)).exp();
        let value_2 = -(-(noise * Self::MOUNTAIN_RANGE_WIDTH_FACTOR - 1.0).powf(2.0)).exp();

        Ok((value_1 + value_2 + 1.0) / 2.0)
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
