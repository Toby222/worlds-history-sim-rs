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
        io::{self, Read, Write},
        path::Path,
    },
};

#[derive(Debug)]
pub enum LoadError {
    MissingSave(io::Error),
    InvalidSave(postcard::Error),
}
impl Error for LoadError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            LoadError::MissingSave(error) => Some(error),
            LoadError::InvalidSave(error) => Some(error),
        }
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}
impl Display for LoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            LoadError::MissingSave(_) => f.write_str("No save found at given path"),
            LoadError::InvalidSave(_) => f.write_str("Loaded file is not a valid save"),
        }
    }
}

#[derive(Debug)]
pub enum SaveError {
    MissingWorld,
    SerializationError(postcard::Error),
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

        let serialized = match postcard::to_stdvec(world) {
            Ok(serialized) => serialized,
            Err(err) => {
                return Err(SaveError::SerializationError(err));
            },
        };

        match File::create(path).unwrap().write_all(serialized.as_slice()) {
            Ok(_) => Ok(()),
            Err(err) => Err(SaveError::FailedToWrite(err)),
        }
    }

    pub fn load_world<P: AsRef<Path>>(&mut self, path: P) -> Result<(), LoadError> {
        let mut file = match File::open(path) {
            Ok(file) => file,
            Err(err) => {
                return Err(LoadError::MissingSave(err));
            },
        };
        let mut buf = vec![];
        if let Err(err) = file.read_to_end(&mut buf) {
            return Err(LoadError::MissingSave(err));
        };

        match postcard::from_bytes(buf.as_slice()) {
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
            let mut new_world = World::async_new(
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
