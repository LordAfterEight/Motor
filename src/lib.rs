pub use macroquad::color;
pub use macroquad::input;
pub use macroquad::prelude;
pub use macroquad;

/// Creates a new Entity with a custom (zero or more) amount of Modules
///
/// # Example
///
/// ```
/// use motor::{Vector2D, Input, Texture, Module, new_entity};
///
/// let position = Module::Position(Vector2D::default());
/// let controls = Module::Controls(Input::default());
/// let sprite   = Module::Sprite(Texture::load("Path/To/File.png").await);
///
/// let player = new_entity!("Player", position, controls, sprite);
/// ```
#[macro_export]
macro_rules! new_entity {
    ( $name:tt ) => {
        {
            let new_entity = $crate::Entity {
                name: $name.to_string(),
                val1: Default::default(),
                val2: Default::default(),
                id: $crate::prelude::rand::rand(),
                modules: None
            };
            new_entity
        }
    };
    ( $name:expr, $($module:expr), *) => {
        {
            let mut new_entity = $crate::Entity {
                name: $name.to_string(),
                val1: Default::default(),
                val2: Default::default(),
                id: $crate::prelude::rand::rand(),
                modules: Some(Vec::new())
            };
            $(
                new_entity.add_module($module);
            )*
            new_entity
        }
    };
}


/// Provides 2D coordinates. Use this in ```Module::Position()```
///
/// # Example
/// ```
/// use motor::Vector2D;
///
/// let position = Module::Position(Vector2D::default());
/// ```
#[derive(Default, Debug, Clone)]
pub struct Vector2D {
    pub x: f32,
    pub y: f32
}

/// Provides an image texture loaded from a file. Use this in ```Module::Sprite()```
///
/// # Example
/// ```
/// use motor::Input;
///
/// let sprite = Module::Sprite(Texture::load("Path/To/File.png").await);
/// ```
#[derive(Debug, Clone)]
pub struct Texture {
    pub texture: macroquad::prelude::Texture2D
}

impl Texture {
    pub async fn load(path: &str) -> Self {
        Self {
            texture: prelude::load_texture(&format!("{}",path) as &str).await.expect("Failed to load texture")
        }
    }
}

impl Default for Texture {
    fn default() -> Self {
        Self {
            texture: macroquad::prelude::Texture2D::empty()
        }
    }
}

/// Provides keyboard input to an Entity. Use this in ```Module::Controls()```
///
/// # Example
/// ```
/// use motor::Controls;
///
/// let controls = Module::Controls(Input::default());
/// ```
#[derive(Debug, Clone)]
pub struct Input {
    pub x_value: f32,
    pub y_value: f32
}

impl Input {
    fn update(&mut self) {

        self.x_value = 0.0;
        self.y_value = 0.0;

        if macroquad::input::is_key_down(macroquad::input::KeyCode::W) {
            println!("W pressed");
            self.y_value =  1.0;
        }
        if macroquad::input::is_key_down(macroquad::input::KeyCode::A) {
            println!("A pressed");
            self.x_value = -1.0;
        }
        if macroquad::input::is_key_down(macroquad::input::KeyCode::S) {
            println!("S pressed");
            self.y_value = -1.0;
        }
        if macroquad::input::is_key_down(macroquad::input::KeyCode::D) {
            println!("D pressed");
            self.x_value =  1.0;
        }
    }
}

impl Default for Input {
    fn default() -> Self {
        Self {
            x_value: 0.0,
            y_value: 0.0
        }
    }
}

#[derive(Debug, Clone)]
pub enum Module {
    /// A position module containing a ```Vector2D```. This provides coordinates to an ```Entity```.
    /// This takes a ```Vector2D```
    ///
    /// # Example
    ///
    ///```
    /// use motor::Vector2D;
    ///
    /// let position = Module::Sprite(Vector2D::default());
    /// ```
    Position(Vector2D),

    /// A sprite module containing an image. This provides a texture to an ```Entity```.
    /// This takes a ```Texture```. Load one directly using ```Texture::load()```
    ///
    /// # Example
    ///
    ///```
    /// use motor::Texture;
    ///
    /// let sprite = Module::Sprite(Texture::load("Path/To/File.png").await);
    /// ```
    Sprite(Texture),

    /// A controls module. This provides keyboard input to an ```Entity```.
    ///
    /// # Example
    ///
    ///```
    /// use motor::Input;
    ///
    /// let sprite = Module::Sprite(Input::default());
    /// ```
    Controls(Input)
}


/// An Entity class that can have several modules
pub struct Entity {
    pub name: String,
    pub id: u32,
    pub val1: f32,
    pub val2: f32,
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
                    Module::Sprite(sprite) => {
                        macroquad::prelude::draw_texture(&sprite.texture, self.val1, self.val2, macroquad::color::WHITE);
                    }
                    Module::Controls(controls) => {
                        controls.update();
                        self.val1 += controls.x_value;
                        self.val2 -= controls.y_value;
                    }
                    Module::Position(vector2d) => {
                        vector2d.x += self.val1;
                        vector2d.y += self.val2;
                    },
                }
            }
        }
    }
}
