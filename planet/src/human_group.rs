use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HumanGroup {
    pub id:         u32,
    pub population: u32,
}
