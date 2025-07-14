use macroquad::prelude::*;
use motor;
use motor::Entity;

#[macroquad::main("Testing")]
async fn main() {
    println!("Hello, world!");

    let player = motor::PlayerEntity::new("Player", (10.0,10.0));
    let module1 = motor::Module::new("Position");
    let module2 = motor::Module::new("Controls");
    let module3 = motor::Module::new("Sprite");
    let module4 = motor::Module::new("Audio");
    let entity = motor::new_entity!("Entity", module1, module2, module3, module4);

    loop {
        clear_background(macroquad::color::PURPLE);
        if motor::input::is_key_pressed(motor::input::KeyCode::Escape) {
            println!("Esc pressed");
            std::process::exit(0)
        }
        next_frame().await
    }
}
