use {
    crate::World,
    core::hash::Hash,
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, Eq, PartialEq, Hash)]
pub struct HumanGroup {
    pub id:         u32,
    pub population: u32,
}

impl HumanGroup {
    pub fn update(&self, _world: &World) {
        // TODO: Anything
    }
}
