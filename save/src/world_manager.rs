use crate::{World, WorldGenError};
use rand::random;

#[derive(Debug)]
pub struct WorldManager {
    world: Option<World>,
}
impl WorldManager {
    pub fn new() -> WorldManager {
        WorldManager { world: None }
    }
    pub fn get_world(&self) -> Option<&World> {
        self.world.as_ref()
    }
    pub fn new_world(&mut self) -> Result<&World, WorldGenError> {
        let mut new_world = World::new(800, 600, random());
        new_world.generate()?;
        self.world = Some(new_world);
        Ok(self.get_world().unwrap())
    }
}
