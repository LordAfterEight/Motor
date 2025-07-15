use macroquad::{color, prelude::*};
use macroquad::texture::Image;
use log;
pub use macroquad::input;
pub use macroquad::prelude;

/// This macro creates a new Entity with a custom (zero or more) amount of Modules
#[macro_export]
macro_rules! new_entity {
    ( $name:tt ) => {
        {
            #[cfg(debug_assertions)]
            log::debug!("Created new Entity instance with name '{}'", $name);
            let new_entity = $crate::Entity {
                name: $name.to_string(),
                id: macroquad::rand::rand(),
                modules: None
            };
            new_entity
        }
    };
    ( $name:expr, $($module:expr), *) => {
        {
            #[cfg(debug_assertions)]
            log::debug!("Created new Entity instance with name '{}' and module(s):", $name);
            let mut new_entity = $crate::Entity {
                name: $name.to_string(),
                id: macroquad::rand::rand(),
                modules: Some(Vec::new())
            };
            $(
                #[cfg(debug_assertions)]
                debug!("  - {:?}", $module);
                new_entity.add_module($module);
            )*
            new_entity
        }
    };
}


/// Provides 2D coordinates. Use this in ```Module::Position()```
#[derive(Default, Debug)]
pub struct Vector2D {
    pub x: f32,
    pub y: f32
}

/// Provides an image texture loaded from a file. Use this in ```Module::Texture()```
#[derive(Debug)]
pub struct Texture {
    pub texture: Texture2D
}

impl Texture {
    pub async fn load(path: &str) -> Self {
        Self {
            texture: load_texture(path).await.expect("Failed to load texture")
        }
    }
}

impl Default for Texture {
    fn default() -> Self {
        Self {
            texture: Texture2D::empty()
        }
    }
}

/// Provides keyboard and mouse input. Use this in ```Module::Controls()```
#[derive(Debug)]
pub struct Input {
}

#[derive(Debug)]
pub enum Module {
    /// A position module containing a ```Vector2D```. This provides coordinates to an ```Entity```
    Position(Vector2D),
    /// A sprite module containing an image. This provides a texture to an ```Entity```
    Sprite(Texture),
    Controls(Input)
}


/// An Entity class that can have several modules (Just an idea I'm testing rn)
pub struct Entity {
    pub name: String,
    pub id: u32,
    pub modules: Option<Vec<Module>>,
}


// --- Entity functions ---

impl Entity {
    /// This creates a new Entity instance with a given name and without modules
    pub fn new(name: &str) -> Self {
        let return_value = new_entity!(name);
        return return_value
    }

    /// This adds a Module to the Entity
    pub fn add_module(&mut self, new_module: Module) {
        #[cfg(debug_assertions)]
        debug!("Added {:?} module to Entity '{}'", new_module, self.name);
        if let Some(modules) = &mut self.modules {
            modules.push(new_module);
        } else {
            self.modules = Some(vec![new_module]);
        }
    }

    pub fn update(&mut self) {
        if let Some(modules) = &mut self.modules {
            for module in modules {
                match module {
                    Module::Position(vector2d) => {
                    },
                    Module::Sprite(sprite) => {
                        let (mut pos_x, mut pos_y) = (0.0,0.0);
                        draw_texture(&sprite.texture, pos_x, pos_y, color::WHITE);
                    },
                    Module::Controls(controls) => {
                    }
                }
            }
        }
    }
}
