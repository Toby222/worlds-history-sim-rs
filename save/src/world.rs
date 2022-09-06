use std::{
    error::Error,
    f32::consts::{PI, TAU},
    fmt::{Debug, Display},
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
            WorldGenError::CartesianError(err) => Display::fmt(err, f),
        }
    }
}

#[derive(Clone)]
pub struct World {
    pub width: i32,
    pub height: i32,
    pub seed: u32,

    pub terrain: Vec<Vec<TerrainCell>>,
    continent_offsets: [Vec2; World::NUM_CONTINENTS as usize],
    continent_widths: [f32; World::NUM_CONTINENTS as usize],
    perlin: Perlin,
}
impl Debug for World {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("World")
            .field("width", &self.width)
            .field("height", &self.height)
            .field("seed", &self.seed)
            .field(
                "Average Rainfall",
                &(self
                    .terrain
                    .iter()
                    .flatten()
                    .map(|cell| cell.rainfall)
                    .sum::<f32>()
                    / (self.width * self.height) as f32),
            )
            .field("continent_offsets", &self.continent_offsets)
            .field("continent_widths", &self.continent_widths)
            .field("perlin", &self.perlin)
            .finish()
    }
}

#[derive(Debug, Clone, Default)]
pub struct Biome {
    pub altitude: f32,
    pub rainfall: f32,
    pub temperature: f32,
}

#[derive(Debug, Clone, Default)]
pub struct TerrainCell {
    pub altitude: f32,
    pub rainfall: f32,
    pub temperature: f32,

    pub rain_accumulated: f32,
    pub previous_rain_accumulated: f32,
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
            continent_offsets: [default(); Self::NUM_CONTINENTS as usize],
            continent_widths: [default(); Self::NUM_CONTINENTS as usize],
            perlin: Perlin::new().set_seed(seed),
        }
    }

    pub const NUM_CONTINENTS: u8 = 7;
    pub const CONTINENT_FACTOR: f32 = 0.7;
    pub const CONTINENT_MIN_WIDTH_FACTOR: f32 = 3.0;
    pub const CONTINENT_MAX_WIDTH_FACTOR: f32 = 7.0;

    pub const MIN_ALTITUDE: f32 = -10000.0;
    pub const MAX_ALTITUDE: f32 = 10000.0;
    pub const ALTITUDE_SPAN: f32 = Self::MAX_ALTITUDE - Self::MIN_ALTITUDE;

    pub const MOUNTAIN_RANGE_MIX_FACTOR: f32 = 0.075;
    pub const MOUNTAIN_RANGE_WIDTH_FACTOR: f32 = 25.0;

    pub const TERRAIN_NOISE_FACTOR_1: f32 = 0.2;
    pub const TERRAIN_NOISE_FACTOR_2: f32 = 0.15;
    pub const TERRAIN_NOISE_FACTOR_3: f32 = 0.1;

    pub const MIN_RAINFALL: f32 = -20.0;
    pub const MAX_RAINFALL: f32 = 100.0;
    pub const RAINFALL_SPAN: f32 = Self::MAX_RAINFALL - Self::MIN_RAINFALL;
    pub const RAINFALL_ALTITUDE_FACTOR: f32 = 1.0;

    pub const MIN_TEMPERATURE: f32 = -100.0;
    pub const MAX_TEMPERATURE: f32 = 100.0;
    pub const TEMPERATURE_SPAN: f32 = Self::MAX_TEMPERATURE - Self::MIN_RAINFALL;
    pub const TEMPERATURE_ALTITUDE_FACTOR: f32 = 1.0;

    pub fn generate(&mut self) -> Result<(), WorldGenError> {
        if let Err(err) = self.generate_altitude() {
            return Err(WorldGenError::CartesianError(err));
        }
        if let Err(err) = self.generate_rainfall() {
            return Err(WorldGenError::CartesianError(err));
        }
        if let Err(err) = self.generate_temperature() {
            return Err(WorldGenError::CartesianError(err));
        }

        Ok(())
    }

    fn generate_continents(&mut self) {
        let mut rng = rand::thread_rng();
        let width = self.width as f32;
        let height = self.height as f32;

        for i in 0..Self::NUM_CONTINENTS {
            self.continent_offsets[i as usize].x = rng
                .gen_range(width * i as f32 * 2.0 / 5.0..width * (i as f32 + 2.0) * 2.0 / 5.0)
                .repeat(width);
            self.continent_offsets[i as usize].y =
                rng.gen_range(height * 1.0 / 6.0..height * 5.0 / 6.0);

            self.continent_widths[i as usize] =
                rng.gen_range(Self::CONTINENT_MIN_WIDTH_FACTOR..Self::CONTINENT_MAX_WIDTH_FACTOR);
        }
    }

    fn continent_modifier(&self, x: usize, y: usize) -> f32 {
        let x = x as f32;
        let y = y as f32;
        let width = self.width as f32;
        let height = self.height as f32;

        let mut max_value = 0.0;
        let beta_factor = f32::sin(PI * y / height);

        for i in 0..Self::NUM_CONTINENTS {
            let idx = i as usize;
            let Vec2 {
                x: continent_x,
                y: continent_y,
            } = self.continent_offsets[idx];

            let distance_x = f32::min(
                f32::min((continent_x - x).abs(), (width + continent_x - x).abs()),
                (continent_x - x - width).abs(),
            ) * beta_factor;

            let distance_y = f32::abs(continent_y - y);

            let distance = (distance_x * distance_x + distance_y * distance_y).sqrt();

            let value = f32::max(0.0, 1.0 - self.continent_widths[idx] * distance / width);

            max_value = f32::max(max_value, value);
        }

        max_value
    }

    fn generate_altitude(&mut self) -> Result<(), CartesianError> {
        self.generate_continents();

        const RADIUS_1: f32 = 0.5;
        const RADIUS_2: f32 = 4.0;
        const RADIUS_3: f32 = 4.0;
        const RADIUS_4: f32 = 8.0;
        const RADIUS_5: f32 = 16.0;

        let offset_1 = Self::random_offset_vector();
        let offset_2 = Self::random_offset_vector();
        let offset_3 = Self::random_offset_vector();
        let offset_4 = Self::random_offset_vector();
        let offset_5 = Self::random_offset_vector();

        for y in 0..self.terrain.len() {
            let alpha = (y as f32 / self.height as f32) * PI;

            for x in 0..self.terrain[y].len() {
                let beta = (x as f32 / self.width as f32) * TAU;

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

                let continent_value = self.continent_modifier(x, y);

                let mut value_continent = continent_value;
                value_continent =
                    mix_values(value_continent, value_3, Self::TERRAIN_NOISE_FACTOR_1);
                value_continent *= mix_values(1.0, value_4, Self::TERRAIN_NOISE_FACTOR_2);

                let value_b = mix_values(1.0, value_5, Self::TERRAIN_NOISE_FACTOR_3);

                let mut value_mountain =
                    mix_values(value_1, value_2, Self::MOUNTAIN_RANGE_MIX_FACTOR);
                value_mountain = self.mountain_range_noise_from_random_noise(value_mountain);
                value_mountain =
                    mix_values(value_mountain, value_3, Self::TERRAIN_NOISE_FACTOR_1 * 1.5);
                value_mountain *= mix_values(1.0, value_4, Self::TERRAIN_NOISE_FACTOR_2 * 1.5);
                value_mountain *= mix_values(1.0, value_5, Self::TERRAIN_NOISE_FACTOR_3 * 1.5);

                let mut raw_altitude = mix_values(value_continent, value_mountain, 0.25);
                raw_altitude = mix_values(raw_altitude, value_mountain, 0.1);
                raw_altitude = mix_values(raw_altitude, value_b, 0.1);

                self.terrain[y][x].altitude = Self::calculate_altitude(raw_altitude);
            }
        }
        Ok(())
    }

    fn random_offset_vector() -> Vec3A {
        random_point_in_sphere(1000.0)
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

    fn mountain_range_noise_from_random_noise(&self, noise: f32) -> f32 {
        let noise = noise * 2.0 - 1.0;

        let value_1 = -f32::exp(-f32::powi(
            noise * Self::MOUNTAIN_RANGE_WIDTH_FACTOR + 1.0,
            2,
        ));
        let value_2 = f32::exp(-f32::powi(
            noise * Self::MOUNTAIN_RANGE_WIDTH_FACTOR - 1.0,
            2,
        ));
        let value_3 = -f32::exp(-f32::powi(
            noise * Self::MOUNTAIN_RANGE_WIDTH_FACTOR + Self::MOUNTAIN_RANGE_WIDTH_FACTOR / 2.0,
            2,
        ));
        let value_4 = f32::exp(-f32::powi(
            noise * Self::MOUNTAIN_RANGE_WIDTH_FACTOR - Self::MOUNTAIN_RANGE_WIDTH_FACTOR / 2.0,
            2,
        ));

        (value_1 + value_2 + value_3 + value_4 + 1.0) / 2.0
    }

    fn calculate_altitude(raw_altitude: f32) -> f32 {
        Self::MIN_ALTITUDE + (raw_altitude * Self::ALTITUDE_SPAN)
    }

    /*
    fn generate_rainfall_alt(&mut self) -> Result<(), CartesianError> {
        let max_cycles = self.width / 5;

        const ACCUMULATED_RAIN_FACTOR: f32 = 0.06;
        const RAINFALL_FACTOR: f32 = 0.005;
        const RAINFALL_ALTITUDE_FACTOR: f32 = 0.05;

        for _ in 0..max_cycles {
            for x in 0..self.width {
                for y in 0..self.height {
                    let mut prev_x = (x - 1 + self.width) % self.width;
                    let prev_y = if y < self.height / 4 {
                        y + 1
                    } else if y < self.height / 2 {
                        prev_x = (x + 1) % self.width;
                        y - 1
                    } else if y < (self.height * 3) / 4 {
                        prev_x = (x + 1) % self.width;
                        y + 1
                    } else {
                        y - 1
                    };

                    let mut cell = self.terrain[y as usize][x as usize];
                    cell.previous_rain_accumulated = cell.rain_accumulated;
                    cell.rain_accumulated = 0.0;

                    if cell.altitude <= 0.0 {
                        cell.rain_accumulated += ACCUMULATED_RAIN_FACTOR * Self::MAX_RAINFALL;
                    }

                    let prev_cell = self.terrain[prev_y as usize][prev_x as usize];

                    let altitude_difference =
                        f32::max(0.0, cell.altitude) - f32::max(0.0, prev_cell.altitude);
                    let altitude_factor = f32::max(
                        0.0,
                        RAINFALL_ALTITUDE_FACTOR * altitude_difference / Self::MAX_ALTITUDE,
                    );
                    let final_rain_factor = f32::min(1.0, RAINFALL_FACTOR + altitude_factor);

                    cell.rain_accumulated += prev_cell.previous_rain_accumulated;
                    cell.rain_accumulated =
                        f32::min(cell.rain_accumulated, Self::MAX_RAINFALL / RAINFALL_FACTOR);

                    let rain_accumulated = cell.rain_accumulated * final_rain_factor;
                    cell.rainfall += rain_accumulated;
                    cell.rain_accumulated -= rain_accumulated;

                    cell.rain_accumulated = f32::max(cell.rain_accumulated, 0.0);

                    self.terrain[y as usize][x as usize] = cell;
                }
            }
        }

        Ok(())
    }
    */

    fn generate_rainfall(&mut self) -> Result<(), CartesianError> {
        let offset = Self::random_offset_vector();
        const RADIUS: f32 = 2.0;

        for y in 0..self.terrain.len() {
            let alpha = (y as f32 / self.height as f32) * PI;
            for x in 0..self.terrain[y].len() {
                let beta = (x as f32 / self.width as f32) * TAU;

                let random_noise =
                    self.random_noise_from_polar_coordinates(alpha, beta, RADIUS, offset)?;

                let mut cell = &mut self.terrain[y][x];

                let base_rainfall = Self::calculate_rainfall(random_noise);
                let altitude_factor = f32::clamp(
                    (cell.altitude / Self::MAX_ALTITUDE) * Self::RAINFALL_ALTITUDE_FACTOR,
                    0.0,
                    1.0,
                );
                let rainfall = base_rainfall * (1.0 - altitude_factor);

                cell.rainfall = rainfall;
            }
        }
        Ok(())
    }

    fn calculate_rainfall(raw_rainfall: f32) -> f32 {
        f32::clamp(
            (raw_rainfall * Self::RAINFALL_SPAN) + Self::MIN_RAINFALL,
            0.0,
            Self::MAX_RAINFALL,
        )
    }

    fn generate_temperature(&mut self) -> Result<(), CartesianError> {
        let offset = Self::random_offset_vector();
        const RADIUS: f32 = 2.0;

        for y in 0..self.terrain.len() {
            let alpha = (y as f32 / self.height as f32) * PI;
            for x in 0..self.terrain[y].len() {
                let beta = (x as f32 / self.width as f32) * TAU;

                let random_noise =
                    self.random_noise_from_polar_coordinates(alpha, beta, RADIUS, offset)?;

                let cell = &mut self.terrain[y][x];

                let altitude_factor = 1.0
                    - f32::clamp(
                        (cell.altitude / Self::MAX_ALTITUDE) * Self::TEMPERATURE_ALTITUDE_FACTOR,
                        0.0,
                        1.0,
                    );

                let latitude_modifer = (alpha * 0.8) + (random_noise * 0.2 * PI);
                let base_temperature = Self::calculate_temperature(f32::sin(latitude_modifer));

                cell.temperature = base_temperature * altitude_factor;
            }
        }

        Ok(())
    }

    fn calculate_temperature(raw_temperature: f32) -> f32 {
        f32::clamp(
            (raw_temperature * Self::TEMPERATURE_SPAN) + Self::MIN_TEMPERATURE,
            0.0,
            Self::MAX_TEMPERATURE,
        )
    }
}
