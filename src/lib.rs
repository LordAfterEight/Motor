use macroquad::prelude::*;

/// A simple player class
pub struct PlayerEntity {
    name: String,
    pos_x: f32,
    pos_y: f32,
}

impl PlayerEntity {
    /// This creates a new player entity instance with a given name and position
    pub fn new(name: &str, position: (f32, f32)) -> Self {
        Self {
            name: name.to_string(),
            pos_x: position.0,
            pos_y: position.1
        }
    }
}
