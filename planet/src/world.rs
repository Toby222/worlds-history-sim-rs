use {
    crate::{
        math_util::{
            cartesian_coordinates,
            mix_values,
            random_point_in_sphere,
            CartesianError,
            RepeatNum,
        },
        perlin,
        BiomeStats,
        BiomeType,
    },
    bevy::{
        log::info,
        math::Vec3A,
        prelude::Vec2,
        utils::{default, HashMap},
    },
    crossbeam_channel::Sender,
    rand::{rngs::StdRng, Rng, SeedableRng},
    serde::{Deserialize, Serialize},
    std::{
        error::Error,
        f32::consts::{PI, TAU},
        fmt::{Debug, Display},
    },
};

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
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            WorldGenError::CartesianError(err) => Display::fmt(err, f),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CompassDirection {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

#[derive(Debug, Serialize)]
pub struct World {
    pub width:  u32,
    pub height: u32,
    pub seed:   u32,

    pub terrain:           Vec<Vec<TerrainCell>>,
    pub continent_offsets: [Vec2; World::NUM_CONTINENTS as usize],
    pub continent_sizes:   [Vec2; World::NUM_CONTINENTS as usize],
    #[serde(skip)]
    pub max_altitude:      f32,
    #[serde(skip)]
    pub min_altitude:      f32,
    #[serde(skip)]
    pub max_rainfall:      f32,
    #[serde(skip)]
    pub min_rainfall:      f32,
    #[serde(skip)]
    pub max_temperature:   f32,
    #[serde(skip)]
    pub min_temperature:   f32,
    #[serde(skip)]
    pub rng:               StdRng,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TerrainCell {
    pub altitude:    f32,
    pub rainfall:    f32,
    pub temperature: f32,

    #[serde(skip)]
    pub x: usize,
    #[serde(skip)]
    pub y: usize,

    pub biome_presences: Vec<(BiomeType, f32)>,
}

impl World {
    pub const ALTITUDE_SPAN: f32 = World::MAX_ALTITUDE - World::MIN_ALTITUDE;
    pub const CONTINENT_MAX_SIZE_FACTOR: f32 = 6.0;
    pub const CONTINENT_MIN_SIZE_FACTOR: f32 = 2.5;
    pub const MAX_ALTITUDE: f32 = 15000.0;
    pub const MAX_RAINFALL: f32 = 7500.0;
    pub const MAX_TEMPERATURE: f32 = 30.0;
    pub const MIN_ALTITUDE: f32 = -15000.0;
    pub const MIN_RAINFALL: f32 = 0.0;
    pub const MIN_TEMPERATURE: f32 = -35.0;
    pub const MOUNTAIN_RANGE_MIX_FACTOR: f32 = 0.075;
    pub const MOUNTAIN_RANGE_WIDTH_FACTOR: f32 = 25.0;
    pub const NUM_CONTINENTS: u8 = 7;
    pub const RAINFALL_DRYNESS_FACTOR: f32 = 0.005;
    pub const RAINFALL_DRYNESS_OFFSET: f32 = World::RAINFALL_DRYNESS_FACTOR * World::MAX_RAINFALL;
    pub const RAINFALL_SPAN: f32 = World::MAX_RAINFALL - World::MIN_RAINFALL;
    pub const TEMPERATURE_ALTITUDE_FACTOR: f32 = 1.0;
    pub const TEMPERATURE_SPAN: f32 = World::MAX_TEMPERATURE - World::MIN_TEMPERATURE;
    pub const TERRAIN_NOISE_FACTOR_1: f32 = 0.15;
    pub const TERRAIN_NOISE_FACTOR_2: f32 = 0.15;
    pub const TERRAIN_NOISE_FACTOR_3: f32 = 0.1;
    pub const TERRAIN_NOISE_FACTOR_4: f32 = 2.5;

    pub fn new(width: u32, height: u32, seed: u32) -> World {
        World {
            width,
            height,
            seed,
            terrain: vec![
                vec![TerrainCell::default(); width.try_into().unwrap()];
                height.try_into().unwrap()
            ],
            continent_offsets: [default(); World::NUM_CONTINENTS as usize],
            continent_sizes: [default(); World::NUM_CONTINENTS as usize],
            max_altitude: World::MIN_ALTITUDE,
            min_altitude: World::MAX_ALTITUDE,
            max_rainfall: World::MIN_RAINFALL,
            min_rainfall: World::MAX_RAINFALL,
            max_temperature: World::MIN_TEMPERATURE,
            min_temperature: World::MAX_TEMPERATURE,
            rng: StdRng::seed_from_u64(seed as u64),
        }
    }

    pub fn async_new(width: u32, height: u32, seed: u32) -> World {
        World {
            width,
            height,
            seed,
            terrain: vec![
                vec![TerrainCell::default(); width.try_into().unwrap()];
                height.try_into().unwrap()
            ],
            continent_offsets: [default(); World::NUM_CONTINENTS as usize],
            continent_sizes: [default(); World::NUM_CONTINENTS as usize],
            max_altitude: World::MIN_ALTITUDE,
            min_altitude: World::MAX_ALTITUDE,
            max_rainfall: World::MIN_RAINFALL,
            min_rainfall: World::MAX_RAINFALL,
            max_temperature: World::MIN_TEMPERATURE,
            min_temperature: World::MAX_TEMPERATURE,
            rng: StdRng::seed_from_u64(seed as u64),
        }
    }

    pub fn generate(
        &mut self,
        progress_sender: &Sender<(f32, String)>,
    ) -> Result<(), WorldGenError> {
        send_progress(progress_sender, 0.0, "Generating altitude");
        if let Err(err) = self.generate_altitude(progress_sender) {
            return Err(WorldGenError::CartesianError(err));
        }
        send_progress(progress_sender, 0.0, "Generating rainfall");
        if let Err(err) = self.generate_rainfall(progress_sender) {
            return Err(WorldGenError::CartesianError(err));
        }
        send_progress(progress_sender, 0.0, "Generating temperature");
        if let Err(err) = self.generate_temperature(progress_sender) {
            return Err(WorldGenError::CartesianError(err));
        }

        send_progress(progress_sender, 0.0, "Generating biomes");
        self.generate_biomes(progress_sender);

        Ok(())
    }

    fn generate_continents(&mut self) {
        #[cfg(feature = "logging")]
        info!("Generating continents");
        let width = self.width as f32;
        let height = self.height as f32;

        for i in 0..World::NUM_CONTINENTS {
            // #[cfg(feature = "logging")]
            // info!("Continents: {}/{}", i, World::NUM_CONTINENTS);

            self.continent_offsets[i as usize].x = self
                .rng
                .gen_range(width * i as f32 * 2.0 / 5.0..(width * (i as f32 + 2.0) * 2.0 / 5.0))
                .repeat(width);
            self.continent_offsets[i as usize].y =
                self.rng.gen_range(height * 1.0 / 6.0..height * 5.0 / 6.0);

            self.continent_sizes[i as usize] = Vec2 {
                x: self
                    .rng
                    .gen_range(World::CONTINENT_MIN_SIZE_FACTOR..World::CONTINENT_MAX_SIZE_FACTOR),
                y: self
                    .rng
                    .gen_range(World::CONTINENT_MIN_SIZE_FACTOR..World::CONTINENT_MAX_SIZE_FACTOR),
            };
        }
        info!("Done generating continents");
    }

    fn continent_modifier(&self, x: usize, y: usize) -> f32 {
        let x = x as f32;
        let y = y as f32;
        let width = self.width as f32;
        let height = self.height as f32;

        let mut max_value = 0.0;
        let beta_factor = f32::sin(PI * y / height);

        for i in 0..World::NUM_CONTINENTS {
            let idx = i as usize;
            let Vec2 {
                x: offset_x,
                y: offset_y,
            } = self.continent_offsets[idx];
            let Vec2 {
                x: continent_width,
                y: continent_height,
            } = self.continent_sizes[idx];

            let distance_x = f32::min(
                f32::min(f32::abs(offset_x - x), f32::abs(width + offset_x - x)),
                f32::abs(offset_x - x - width),
            ) * beta_factor
                * continent_width;

            let distance_y = f32::abs(offset_y - y) * continent_height;

            let distance = f32::sqrt((distance_x * distance_x) + (distance_y * distance_y));

            let value = f32::max(0.0, 1.0 - distance / width);

            if value > max_value {
                max_value = value;
            }
        }

        max_value
    }

    fn generate_altitude(
        &mut self,
        progress_sender: &Sender<(f32, String)>,
    ) -> Result<(), CartesianError> {
        info!("Generating altitude");
        self.generate_continents();

        const RADIUS_1: f32 = 0.5;
        const RADIUS_2: f32 = 4.0;
        const RADIUS_3: f32 = 4.0;
        const RADIUS_4: f32 = 8.0;
        const RADIUS_5: f32 = 16.0;

        let offset_1 = World::random_offset_vector(&mut self.rng);
        let offset_2 = World::random_offset_vector(&mut self.rng);
        let offset_3 = World::random_offset_vector(&mut self.rng);
        let offset_4 = World::random_offset_vector(&mut self.rng);
        let offset_5 = World::random_offset_vector(&mut self.rng);

        let height = self.terrain.len();
        for y in 0..height {
            let alpha = (y as f32 / self.height as f32) * PI;

            let width = self.terrain[y].len();
            let size = height * width;
            for x in 0..width {
                let index = y * width + x;
                send_progress(
                    progress_sender,
                    index as f32 / size as f32,
                    format!("Generating topography: {index}/{size}"),
                );

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

                let value_a = mix_values(
                    self.continent_modifier(x, y),
                    value_3,
                    World::TERRAIN_NOISE_FACTOR_1,
                ) * mix_values(1.0, value_4, World::TERRAIN_NOISE_FACTOR_2)
                    * mix_values(1.0, value_5, World::TERRAIN_NOISE_FACTOR_3);

                let value_b = value_a * 0.04 + 0.48;

                let value_c = mix_values(
                    self.mountain_range_noise_from_random_noise(mix_values(
                        value_1,
                        value_2,
                        World::MOUNTAIN_RANGE_MIX_FACTOR,
                    )),
                    value_3,
                    World::TERRAIN_NOISE_FACTOR_1 * World::TERRAIN_NOISE_FACTOR_4,
                ) * mix_values(
                    1.0,
                    value_4,
                    World::TERRAIN_NOISE_FACTOR_2 * World::TERRAIN_NOISE_FACTOR_4,
                ) * mix_values(
                    1.0,
                    value_5,
                    World::TERRAIN_NOISE_FACTOR_3 * World::TERRAIN_NOISE_FACTOR_4,
                );

                let mut value_d = mix_values(value_a, value_c, 0.25);
                value_d = mix_values(value_d, value_c, 0.1);
                value_d = mix_values(value_d, value_b, 0.1);

                let altitude = World::calculate_altitude(value_d);
                self.terrain[y][x].altitude = altitude;

                if altitude > self.max_altitude {
                    self.max_altitude = altitude;
                }
                if altitude < self.min_altitude {
                    self.min_altitude = altitude;
                }

                self.terrain[y][x].x = x;
                self.terrain[y][x].y = y;
            }
        }
        info!("Done generating altitude");
        Ok(())
    }

    fn random_offset_vector(rng: &mut StdRng) -> Vec3A {
        random_point_in_sphere(rng, 1000.0)
    }

    fn random_noise_from_polar_coordinates(
        &self,
        alpha: f32,
        beta: f32,
        radius: f32,
        offset: Vec3A,
    ) -> Result<f32, CartesianError> {
        let cartesian = cartesian_coordinates(alpha, beta, radius)?;
        Ok(perlin::perlin_value(
            cartesian.x + offset.x,
            cartesian.y + offset.y,
            cartesian.z + offset.z,
        ))
    }

    fn mountain_range_noise_from_random_noise(&self, noise: f32) -> f32 {
        let noise = noise * 2.0 - 1.0;

        let value_1 = -f32::exp(-f32::powi(
            noise * World::MOUNTAIN_RANGE_WIDTH_FACTOR + 1.0,
            2,
        ));
        let value_2 = f32::exp(-f32::powi(
            noise * World::MOUNTAIN_RANGE_WIDTH_FACTOR - 1.0,
            2,
        ));
        let value_3 = -f32::exp(-f32::powi(
            noise * World::MOUNTAIN_RANGE_WIDTH_FACTOR + World::MOUNTAIN_RANGE_WIDTH_FACTOR / 2.0,
            2,
        ));
        let value_4 = f32::exp(-f32::powi(
            noise * World::MOUNTAIN_RANGE_WIDTH_FACTOR - World::MOUNTAIN_RANGE_WIDTH_FACTOR / 2.0,
            2,
        ));

        (value_1 + value_2 + value_3 + value_4 + 1.0) / 2.0
    }

    fn calculate_altitude(raw_altitude: f32) -> f32 {
        World::MIN_ALTITUDE + (raw_altitude * World::ALTITUDE_SPAN)
    }

    fn generate_rainfall(
        &mut self,
        progress_sender: &Sender<(f32, String)>,
    ) -> Result<(), CartesianError> {
        info!("Generating rainfall");
        const RADIUS: f32 = 2.0;
        let offset = World::random_offset_vector(&mut self.rng);

        let height = self.terrain.len();
        for y in 0..height {
            let alpha = (y as f32 / self.height as f32) * PI;

            let width = self.terrain[y].len();
            let size = width * height;
            for x in 0..width {
                let index = y * width + x;
                send_progress(
                    progress_sender,
                    index as f32 / size as f32,
                    format!("Generating rainfall: {index}/{size}"),
                );
                let beta = (x as f32 / self.width as f32) * TAU;

                let random_noise =
                    self.random_noise_from_polar_coordinates(alpha, beta, RADIUS, offset)?;

                let latitude_factor = alpha + (random_noise * 2.0 - 1.0) * PI * 0.1;
                let latitude_modifier_1 = (1.5 * f32::sin(latitude_factor)) - 0.5;
                let latitude_modifier_2 = f32::cos(latitude_factor);

                let offset_cell_x =
                    (width + x + f32::floor(latitude_modifier_2 * width as f32 / 20.0) as usize)
                        % width;
                let offset_cell_2_x =
                    (width + x + f32::floor(latitude_modifier_2 * width as f32 / 10.0) as usize)
                        % width;

                let offset_cell = &self.terrain[y][offset_cell_x];
                let offset_altitude = f32::max(0.0, offset_cell.altitude);

                let offset_cell_2 = &self.terrain[y][offset_cell_2_x];
                let offset_altitude_2 = f32::max(0.0, offset_cell_2.altitude);

                let cell = &mut self.terrain[y][x];
                let altitude_value = f32::max(0.0, cell.altitude);

                let altitude_modifier =
                    (altitude_value - (offset_altitude * 1.5) - (offset_altitude_2))
                        / World::MAX_ALTITUDE;

                let rainfall_value = mix_values(latitude_modifier_1, altitude_modifier, 0.63);
                let rainfall_value = mix_values(rainfall_value.powi(2), rainfall_value, 0.85);
                let rainfall = f32::min(
                    World::MAX_RAINFALL,
                    World::calculate_rainfall(rainfall_value),
                );

                cell.rainfall = rainfall;

                if rainfall > self.max_rainfall {
                    self.max_rainfall = rainfall;
                }
                if rainfall < self.min_rainfall {
                    self.min_rainfall = rainfall;
                }
            }
        }
        info!("Done generating rainfall");
        Ok(())
    }

    fn calculate_rainfall(raw_rainfall: f32) -> f32 {
        f32::clamp(
            (raw_rainfall * (World::RAINFALL_SPAN + World::RAINFALL_DRYNESS_OFFSET))
                + World::MIN_RAINFALL
                - World::RAINFALL_DRYNESS_OFFSET,
            0.0,
            World::MAX_RAINFALL,
        )
    }

    fn generate_temperature(
        &mut self,
        progress_sender: &Sender<(f32, String)>,
    ) -> Result<(), CartesianError> {
        info!("Generating temperature");
        let offset = World::random_offset_vector(&mut self.rng);
        const RADIUS: f32 = 2.0;

        let height = self.terrain.len();
        for y in 0..height {
            let alpha = (y as f32 / self.height as f32) * PI;

            let width = self.terrain[y].len();
            let size = width * height;
            for x in 0..width {
                let index = y * width + x;
                send_progress(
                    progress_sender,
                    index as f32 / size as f32,
                    format!("Generating temperature: {index}/{size}"),
                );

                let beta = (x as f32 / self.width as f32) * TAU;

                let random_noise =
                    self.random_noise_from_polar_coordinates(alpha, beta, RADIUS, offset)?;

                let cell = &mut self.terrain[y][x];

                let latitude_modifer = mix_values(alpha, random_noise * PI, 0.1);
                let altitude_factor = f32::max(
                    0.0,
                    (cell.altitude / World::MAX_ALTITUDE) * World::TEMPERATURE_ALTITUDE_FACTOR,
                );
                let temperature =
                    World::calculate_temperature(f32::sin(latitude_modifer) - altitude_factor);

                cell.temperature = temperature;

                if temperature > self.max_temperature {
                    self.max_temperature = temperature;
                }
                if temperature < self.min_temperature {
                    self.min_temperature = temperature;
                }
            }
        }

        info!("Done generating temperature");
        Ok(())
    }

    fn calculate_temperature(raw_temperature: f32) -> f32 {
        f32::clamp(
            (raw_temperature * World::TEMPERATURE_SPAN) + World::MIN_TEMPERATURE,
            World::MIN_TEMPERATURE,
            World::MAX_TEMPERATURE,
        )
    }

    fn generate_biomes(&mut self, progress_sender: &Sender<(f32, String)>) {
        info!("Generating biomes");
        let height = self.terrain.len();
        for y in 0..height {
            let width = self.terrain[y].len();
            let size = height * width;
            for x in 0..width {
                let index = y * width + x;
                send_progress(
                    progress_sender,
                    index as f32 / size as f32,
                    format!("Generating biomes: {index}/{size}"),
                );
                let cell = &self.terrain[y][x];

                let mut total_presence = 0.0;

                let mut biome_presences = vec![];
                for biome_type in BiomeType::iterator() {
                    let presence = self.biome_presence(cell, &biome_type.into());

                    if presence <= 0.0 {
                        continue;
                    }

                    biome_presences.push((*biome_type, presence));
                    total_presence += presence;
                }
                self.terrain[y][x].biome_presences = biome_presences
                    .iter()
                    .map(|(biome_type, presence)| (*biome_type, presence / total_presence))
                    .collect();
            }
        }
        info!("Done generating biomes");
    }

    fn biome_presence(&self, cell: &TerrainCell, biome: &BiomeStats) -> f32 {
        let mut presence = 0.0;
        let altitude_diff = cell.altitude - biome.min_altitude;
        if altitude_diff < 0.0 {
            return 0.0;
        }

        let altitude_factor = altitude_diff / (biome.max_altitude - biome.min_altitude);
        if altitude_factor > 1.0 {
            return 0.0;
        };

        presence += if altitude_factor > 0.5 {
            1.0 - altitude_factor
        } else {
            altitude_factor
        };

        let rainfall_diff = cell.rainfall - biome.min_rainfall;
        if rainfall_diff < 0.0 {
            return 0.0;
        }

        let rainfall_factor = rainfall_diff / (biome.max_rainfall - biome.min_rainfall);
        if rainfall_factor > 1.0 {
            return 0.0;
        }

        presence += if rainfall_factor > 0.5 {
            1.0 - rainfall_factor
        } else {
            rainfall_factor
        };

        let temperature_diff = cell.temperature - biome.min_temperature;
        if temperature_diff < 0.0 {
            return 0.0;
        }

        let temperature_factor = temperature_diff / (biome.max_temperature - biome.min_temperature);
        if temperature_factor > 1.0 {
            return 0.0;
        }

        presence += if temperature_factor > 0.5 {
            1.0 - temperature_factor
        } else {
            temperature_factor
        };

        presence
    }

    #[must_use]
    pub fn cell_neighbors(&self, x: usize, y: usize) -> HashMap<CompassDirection, &TerrainCell> {
        let mut neighbors = HashMap::new();

        let height = self.height as usize;
        let width = self.width as usize;

        let west_x = (width + x - 1) % width;
        let east_x = (x + 1) % width;

        let north_edge = y >= height - 1;
        let south_edge = y == 0;

        if !north_edge {
            _ = neighbors
                .insert_unique_unchecked(CompassDirection::NorthWest, &self.terrain[y + 1][west_x]);
            _ = neighbors.insert_unique_unchecked(CompassDirection::North, &self.terrain[y + 1][x]);
            _ = neighbors
                .insert_unique_unchecked(CompassDirection::NorthEast, &self.terrain[y + 1][east_x]);
        }

        _ = neighbors.insert_unique_unchecked(CompassDirection::West, &self.terrain[y][east_x]);
        _ = neighbors.insert_unique_unchecked(CompassDirection::East, &self.terrain[y][west_x]);

        if !south_edge {
            _ = neighbors
                .insert_unique_unchecked(CompassDirection::SouthWest, &self.terrain[y - 1][west_x]);
            _ = neighbors.insert_unique_unchecked(CompassDirection::South, &self.terrain[y - 1][x]);
            _ = neighbors
                .insert_unique_unchecked(CompassDirection::SouthEast, &self.terrain[y - 1][east_x]);
        }

        neighbors
    }

    #[must_use]
    pub fn get_slant(&self, cell: &TerrainCell) -> f32 {
        let neighbors = self.cell_neighbors(cell.x, cell.y);

        let mut west_altitude = 0.0;
        let mut neighbor_count = 0u8;

        if let Some(neighbor) = neighbors.get(&CompassDirection::West) {
            west_altitude = f32::max(west_altitude, neighbor.altitude);
            neighbor_count += 1;
        }
        if let Some(neighbor) = neighbors.get(&CompassDirection::SouthWest) {
            west_altitude = f32::max(west_altitude, neighbor.altitude);
            neighbor_count += 1;
        }
        if let Some(neighbor) = neighbors.get(&CompassDirection::South) {
            west_altitude = f32::max(west_altitude, neighbor.altitude);
            neighbor_count += 1;
        }

        west_altitude /= f32::from(neighbor_count);
        neighbor_count = 0;

        let mut east_altitude = f32::MIN;
        if let Some(neighbor) = neighbors.get(&CompassDirection::East) {
            east_altitude = f32::max(east_altitude, neighbor.altitude);
            neighbor_count += 1;
        }
        if let Some(neighbor) = neighbors.get(&CompassDirection::NorthEast) {
            east_altitude = f32::max(east_altitude, neighbor.altitude);
            neighbor_count += 1;
        }
        if let Some(neighbor) = neighbors.get(&CompassDirection::North) {
            east_altitude = f32::max(east_altitude, neighbor.altitude);
            neighbor_count += 1;
        }

        east_altitude /= f32::from(neighbor_count);

        west_altitude - east_altitude
    }

    pub fn is_cell_near_coastline(&self, cell: &TerrainCell) -> bool {
        if cell.altitude >= 0.0 {
            return false;
        }

        let neighbors = self.cell_neighbors(cell.x, cell.y);

        if let Some(neighbor) = neighbors.get(&CompassDirection::West) {
            if neighbor.altitude >= 0.0 {
                return true;
            }
        }
        if let Some(neighbor) = neighbors.get(&CompassDirection::NorthWest) {
            if neighbor.altitude >= 0.0 {
                return true;
            }
        }
        if let Some(neighbor) = neighbors.get(&CompassDirection::North) {
            if neighbor.altitude >= 0.0 {
                return true;
            }
        }
        if let Some(neighbor) = neighbors.get(&CompassDirection::NorthEast) {
            if neighbor.altitude >= 0.0 {
                return true;
            }
        }
        if let Some(neighbor) = neighbors.get(&CompassDirection::East) {
            if neighbor.altitude >= 0.0 {
                return true;
            }
        }
        if let Some(neighbor) = neighbors.get(&CompassDirection::SouthEast) {
            if neighbor.altitude >= 0.0 {
                return true;
            }
        }
        if let Some(neighbor) = neighbors.get(&CompassDirection::South) {
            if neighbor.altitude >= 0.0 {
                return true;
            }
        }
        if let Some(neighbor) = neighbors.get(&CompassDirection::SouthWest) {
            if neighbor.altitude >= 0.0 {
                return true;
            }
        }
        return false;
    }

    #[must_use]
    pub fn is_cell_coastline(&self, cell: &TerrainCell) -> bool {
        if cell.altitude <= 0.0 {
            return false;
        }

        let neighbors = self.cell_neighbors(cell.x, cell.y);

        if let Some(neighbor) = neighbors.get(&CompassDirection::West) {
            if neighbor.altitude <= 0.0 {
                return true;
            }
        }
        if let Some(neighbor) = neighbors.get(&CompassDirection::NorthWest) {
            if neighbor.altitude <= 0.0 {
                return true;
            }
        }
        if let Some(neighbor) = neighbors.get(&CompassDirection::North) {
            if neighbor.altitude <= 0.0 {
                return true;
            }
        }
        if let Some(neighbor) = neighbors.get(&CompassDirection::NorthEast) {
            if neighbor.altitude <= 0.0 {
                return true;
            }
        }
        if let Some(neighbor) = neighbors.get(&CompassDirection::East) {
            if neighbor.altitude <= 0.0 {
                return true;
            }
        }
        if let Some(neighbor) = neighbors.get(&CompassDirection::SouthEast) {
            if neighbor.altitude <= 0.0 {
                return true;
            }
        }
        if let Some(neighbor) = neighbors.get(&CompassDirection::South) {
            if neighbor.altitude <= 0.0 {
                return true;
            }
        }
        if let Some(neighbor) = neighbors.get(&CompassDirection::SouthWest) {
            if neighbor.altitude <= 0.0 {
                return true;
            }
        }
        return false;
    }
}

fn send_progress<T: Into<String>>(
    progress_sender: &Sender<(f32, String)>,
    progress: f32,
    progress_text: T,
) {
    if let Err(_) = progress_sender.try_send((progress, progress_text.into())) {
        // Quietly ignore the error, it's not critical, and logging is slow.

        // debug!("Failed to send world generation progress. {err}");
    }
}
