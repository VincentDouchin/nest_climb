use bevy::prelude::*;

#[derive(Component, PartialEq, Eq)]
pub struct Health {
    pub current_health: u32,
    pub max_health: u32,
}
impl Health {
    pub fn new(max_health: u32) -> Self {
        return Health {
            current_health: max_health,
            max_health,
        };
    }
}
// impl PartialEq for Health {
//     fn ne(&self, other: &Self) -> bool {
//         return &self.current_health != other.current_health
//             || &self.max_health != other.max_health;
//     }
// }
