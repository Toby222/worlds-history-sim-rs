use std::io::{Write, BufReader};

use {
    crate::{World, WorldGenError},
    bevy::{
        log::warn,
        prelude::Resource,
        tasks::{AsyncComputeTaskPool, Task},
        utils::default,
    },
    crossbeam_channel::Sender,
    rand::random,
    std::{
        error::Error,
        fmt::Display,
        fs::File,
        io::{self, BufWriter},
        path::Path,
    },
};

#[derive(Debug)]
pub enum LoadError {
    MissingSave(io::Error),
    InvalidSave(bincode::Error),
}
impl Error for LoadError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            LoadError::MissingSave(error) => Some(error),
            LoadError::InvalidSave(error) => Some(error),
        }
    }
}

impl Display for LoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            LoadError::MissingSave(_) => f.write_str("No save found at given path"),
            LoadError::InvalidSave(err) => f.write_fmt(format_args!(
                "Loaded file is not a valid save - {}",
                err.to_string()
            )),
        }
    }
}

#[derive(Debug)]
pub enum SaveError {
    MissingWorld,
    SerializationError(bincode::Error),
    FailedToWrite(io::Error),
}
impl Error for SaveError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            SaveError::MissingWorld => None,
            SaveError::SerializationError(error) => Some(error),
            SaveError::FailedToWrite(err) => Some(err),
        }
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}
impl Display for SaveError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SaveError::MissingWorld => f.write_str("No world to save found."),
            SaveError::SerializationError(_) => f.write_str("Failed to serialize world."),
            SaveError::FailedToWrite(_) => f.write_str("Failed to write save file."),
        }
    }
}

#[derive(Debug, Default, Resource)]
pub struct WorldManager {
    world: Option<World>,
}

impl WorldManager {
    const NEW_WORLD_HEIGHT: u32 = 200;
    const NEW_WORLD_WIDTH: u32 = 400;

    #[must_use]
    pub fn new() -> WorldManager {
        default()
    }

    pub fn save_world<P: AsRef<Path>>(&self, path: P) -> Result<(), SaveError> {
        let Some(world) = self.get_world() else {
            warn!("No world to save");
            return Err(SaveError::MissingWorld);
        };

        let save_file = match File::create(path) {
            Ok(save_file) => save_file,
            Err(err) => return Err(SaveError::FailedToWrite(err)),
        };

        let serialized = match bincode::serialize(world) {
            Ok(serialized) => serialized,
            Err(err) => return Err(SaveError::SerializationError(err)),
        };

        match BufWriter::new(save_file).write(serialized.as_slice()) {
            Ok(_) => Ok(()),
            Err(err) => Err(SaveError::FailedToWrite(err)),
        }
    }

    pub fn load_world<P: AsRef<Path>>(&mut self, path: P) -> Result<(), LoadError> {
        let file = match File::open(path) {
            Ok(file) => file,
            Err(err) => {
                return Err(LoadError::MissingSave(err));
            },
        };
        
        match bincode::deserialize_from(BufReader::new(file)) {
            Ok(world) => {
                self.world = Some(world);
                Ok(())
            },
            Err(err) => Err(LoadError::InvalidSave(err)),
        }
    }

    #[must_use]
    pub fn get_world(&self) -> Option<&World> {
        self.world.as_ref()
    }

    #[must_use]
    pub fn get_world_mut(&mut self) -> Option<&mut World> {
        self.world.as_mut()
    }

    pub fn set_world(&mut self, world: World) {
        self.world = Some(world);
    }

    pub fn new_world_async(
        &mut self,
        seed: Option<u32>,
        progress_sender: Sender<(f32, String)>,
    ) -> Task<Result<World, WorldGenError>> {
        AsyncComputeTaskPool::get().spawn(async move {
            let seed = seed.unwrap_or_else(random);
            let mut new_world = World::new(
                WorldManager::NEW_WORLD_WIDTH,
                WorldManager::NEW_WORLD_HEIGHT,
                seed,
            );
            if let Err(_) = progress_sender.try_send((0.0, String::from("Generating new world...")))
            {
                // Quietly ignore. It's not critical and logging is slow.
            }
            let result = new_world.generate(&progress_sender);
            if let Err(_) = progress_sender.try_send((1.0, String::from("Done generating world!")))
            {
                // Quietly ignore. See above
            }
            match result {
                Ok(()) => Ok(new_world),
                Err(err) => Err(err),
            }
        })
    }
}
