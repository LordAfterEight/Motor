pub use macroquad;
pub use macroquad::window;
pub use macroquad::input;
pub use macroquad::color;
pub use macroquad::main;
pub use colored::Colorize;

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
            pub use $crate::Colorize;
            let new_entity = $crate::Entity {
                name: $name.to_string(),
                val1: Default::default(),
                val2: Default::default(),
                vals: Vec::new(),
                id: $crate::macroquad::prelude::rand::rand(),
                modules: None
            };
            #[cfg(debug_assertions)]
            println!("{} {} {} {} \"{}\"", "[i]".yellow(), "Created new", "Entity".green(), "with name", new_entity.name.cyan());
            new_entity
        }
    };
    ( $name:expr, $($module:expr), *) => {
        {
            pub use $crate::Colorize;
            let mut new_entity = $crate::Entity {
                name: $name.to_string(),
                val1: Default::default(),
                val2: Default::default(),
                vals: Vec::new(),
                id: $crate::macroquad::prelude::rand::rand(),
                modules: Some(Vec::new())
            };
            #[cfg(debug_assertions)]
            println!("{} {} {} {} \"{}\"", "[i]".yellow(), "Created new", "Entity".green(), "with name", new_entity.name.cyan());
            $(
                #[cfg(debug_assertions)]
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
    /// loads a ```macroquad::texture::Texture2D``` from a file
    ///
    /// # Example
    /// ```
    /// use motor::Texture;
    ///
    /// let texture = Texture::load("Path/To/File.png").await;
    /// ```
    pub async fn load(path: &str) -> Self {
        let texture = match macroquad::prelude::load_texture(&format!("{}",path) as &str).await {
            Ok(texture) => {
                return Self { texture }
            },
            Err(_texture) => {
                println!("{} {} \"./{}\"\n    {}",
                    "[X]".red(),
                    "Failed to load texture from".truecolor(200, 50, 0),
                    path,
                    "Sending empty texture instead".truecolor(200, 50, 0)
                );
                return Self { texture: macroquad::texture::Texture2D::empty() }
            }
        };
    }
}

impl Default for Texture {
    fn default() -> Self {
        Self {
            texture: macroquad::texture::Texture2D::empty()
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

        if macroquad::input::is_key_down(macroquad::input::KeyCode::W) { self.y_value =  1.0; }
        if macroquad::input::is_key_down(macroquad::input::KeyCode::A) { self.x_value = -1.0; }
        if macroquad::input::is_key_down(macroquad::input::KeyCode::S) { self.y_value = -1.0; }
        if macroquad::input::is_key_down(macroquad::input::KeyCode::D) { self.x_value =  1.0; }
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
/// This contains a selection of modules that can be given to any ```Entity``` even at runtime
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
    pub vals: Vec<(String, f32)>,
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
        println!("{} {} {} {:?} {} {} \"{}\"", "[i]".yellow(), "Added", "Module".green(), new_module, "to", "Entity".green(), self.name.cyan());
        if let Some(modules) = &mut self.modules {
            modules.push(new_module);
        } else {
            self.modules = Some(vec![new_module]);
        }
    }

    /// Adds or overwrites an ```Entity```s value using the given identifier and value.
    ///
    /// This function will check automatically if a value is already present. If not, it will
    /// add it. If the value exists, it will overwrite the value connected to the identifier
    ///
    /// # Example
    ///
    /// ```
    /// use motor::{Input, Vector2D, Texture, Module, new_entity}
    ///
    /// let position = Module::Position(Vector2D::default());
    /// let controls = Module::Controls(Input::default());
    /// let sprite   = Module::Sprite(Texture::load("Path/To/File.png").await);
    ///
    /// let player = new_entity!("Player", position, control, sprite);
    ///
    /// // This line adds the value "Speed" to the player Entity. The Entity's
    /// // Controls module will use this value to modulate its outputs.
    /// player.add_value("speed", 3.0);
    /// ```
    pub fn add_value(&mut self, id: &str, val: f32) {
        #[cfg(debug_assertions)]
        println!("{} Added \"{}\" with value {} to {} {}",
            "[i]".yellow(),
            id.magenta(),
            val.to_string().purple(),
            "Entity".green(),
            self.name.cyan()
        );
        self.vals.push((id.to_string(), val));
    }

    pub fn update(&mut self) {
        if let Some(modules) = &mut self.modules {
            for module in modules {
                match module {
                    Module::Sprite(sprite) => {
                        let (mut x, mut y) = (0.0,0.0);
                        for value in self.vals.iter_mut() {
                            if value.0 == "ctrl_x" {
                                x = value.1;
                            }
                            if value.0 == "ctrl_y" {
                                y = value.1;
                            }
                        }
                        macroquad::prelude::draw_texture(&sprite.texture, x, y, macroquad::color::WHITE);
                    }
                    Module::Controls(controls) => {
                        controls.update();
                        for value in self.vals.iter_mut() {
                            if value.0 == "ctrl_x" {
                                value.1 = controls.x_value;
                            } else if value.0 == "ctrl_y" {
                                value.1 = controls.y_value;
                            } else {
                                //self.vals.push(("ctrl_x".to_string(), controls.x_value));
                                //self.vals.push(("ctrl_y".to_string(), controls.y_value));
                            }
                        }
                    }
                    Module::Position(_vector2d) => {
                    },
                }
            }
        }
    }
}
