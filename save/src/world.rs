use std::{
    error::Error,
    f32::consts::{PI, TAU},
    fmt::Display,
};

use noise::{NoiseFn, Perlin, Seedable};

use crate::{cartesian_coordinates, random_point_in_sphere, CartesianError};

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

#[derive(Debug)]
pub struct World {
    pub width: i32,
    pub height: i32,
    pub seed: u32,

    pub terrain: Vec<Vec<TerrainCell>>,
}

#[derive(Debug, Copy, Clone, Default)]
pub struct TerrainCell {
    pub altitude: f32,
    pub rainfall: f32,
}

impl World {
    pub fn new(width: i32, height: i32, seed: u32) -> World {
        let terrain = vec![
            vec![TerrainCell::default(); width.try_into().unwrap()];
            height.try_into().unwrap()
        ];

        World {
            width,
            height,
            seed,
            terrain,
        }
    }

    pub const MIN_ALTITUDE: f32 = -10000.0;
    pub const MAX_ALTITUDE: f32 = 10000.0;
    pub const ALTITUDE_SPAN: f32 = Self::MAX_ALTITUDE - Self::MIN_ALTITUDE;

    pub const MIN_RAINFALL: f32 = -10.0;
    pub const MAX_RAINFALL: f32 = 100.0;
    pub const RAINFALL_SPAN: f32 = Self::MAX_RAINFALL - Self::MIN_RAINFALL;
    pub const RAINFALL_ALTITUDE_FACTOR: f32 = 1.0;

    pub fn generate(&mut self) -> Result<(), WorldGenError> {
        let perlin = Perlin::new().set_seed(self.seed);

        if let Err(err) = self.generate_altitude(&perlin) {
            return Err(WorldGenError::CartesianError(err));
        }
        if let Err(err) = self.generate_rainfall(&perlin) {
            return Err(WorldGenError::CartesianError(err));
        }

        Ok(())
    }
    fn generate_altitude(&mut self, perlin: &Perlin) -> Result<(), CartesianError> {
        let offset = random_point_in_sphere(1000.0);
        const RADIUS: f32 = 2.0;

        for (y, row) in self.terrain.iter_mut().enumerate() {
            let alpha = (y as f32 / self.height as f32) * PI;

            for (x, cell) in row.iter_mut().enumerate() {
                let beta = (x as f32 / self.width as f32) * TAU;
                let pos = cartesian_coordinates(alpha, beta, RADIUS)? + offset;

                let value = Perlin::get(perlin, [pos.x.into(), pos.y.into(), pos.z.into()]) as f32;

                let altitude = Self::MIN_ALTITUDE + (value * Self::ALTITUDE_SPAN);

                cell.altitude = altitude;
            }
        }
        Ok(())
    }

    fn generate_rainfall(&mut self, perlin: &Perlin) -> Result<(), CartesianError> {
        let offset = random_point_in_sphere(1000.0);
        const RADIUS: f32 = 2.0;

        for (y, row) in self.terrain.iter_mut().enumerate() {
            let alpha = (y as f32 / self.height as f32) * PI;
            for (x, cell) in row.iter_mut().enumerate() {
                let beta = (x as f32 / self.width as f32) * TAU;

                let pos = cartesian_coordinates(alpha, beta, RADIUS)? + offset;

                let value = Perlin::get(perlin, [pos.x.into(), pos.y.into(), pos.z.into()]) as f32;

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
}
