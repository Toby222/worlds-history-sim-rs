use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct HumanGroup {
    pub id:         u32,
    pub population: u32,
}
