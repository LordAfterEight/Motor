use macroquad::prelude::*;
use macroquad::texture::Image;
pub use macroquad::input;

/// This macro creates a new Entity with a custom (zero or more) amount of Modules
#[macro_export]
macro_rules! new_entity {
    ( $name:tt ) => {
        {
            #[cfg(debug_assertions)]
            debug!("Created new Entity instance with name {}", $name);
            let new_entity = crate::Entity {
                name: $name.to_string(),
                ID: macroquad::rand::rand(),
                modules: None
            };
            new_entity
        }
    };
    ( $name:expr, $($module:expr), *) => {
        {
            #[cfg(debug_assertions)]
            debug!("Created new Entity instance with name {} and module(s):", $name);
            let mut module_vec = Vec::new();
            $(
                debug!("  - {}", $module.name);
                module_vec.push($module);

            )*
            let mut new_entity = crate::Entity {
                name: $name.to_string(),
                ID: macroquad::rand::rand(),
                modules: Some(module_vec)
            };
            new_entity
        }
    };
}


/// A simple player class
pub struct PlayerEntity {
    pub name: String,
    pub sprite: Option<Image>,
    pub pos_x: f32,
    pub pos_y: f32,
}

#[derive(Clone)]
pub struct Module {
    pub name: String
}


/// An Entity class that can have several modules (Just an idea I'm testing rn)
///
///
pub struct Entity {
    pub name: String,
    pub ID: u32,
    pub modules: Option<Vec<Module>>,
}


// --- PlayerEntity functions ---

impl PlayerEntity {
    /// This creates a new PlayerEntity instance with a given name and position
    pub fn new(name: &str, position: (f32, f32)) -> Self {
        Self {
            name: name.to_string(),
            sprite: None,
            pos_x: position.0,
            pos_y: position.1
        }
    }
}


// --- Entity functions ---

impl Entity {
    /// This creates a new Entity instance with a given name and without modules
    pub fn new(name: &str) -> Self {
        let return_value = new_entity!("Entity");
        return return_value
    }

    /// This adds a Module to the Entity
    pub fn add_module(&mut self, module: Module) {
        if self.modules.is_none() {
            self.modules = Some(vec![module]);
        } else {
            let mut temp_vec = self.modules.clone().unwrap();
            temp_vec.push(module);
            self.modules = Some(temp_vec);
        }
    }
}


// --- Module functions ---

impl Module {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

