use macroquad::prelude::*;
use motor;
use motor::Entity;

#[macroquad::main("Testing")]
async fn main() {
    println!("Hello, world!");

    let player = motor::PlayerEntity::new("Player", (10.0,10.0));
    let module1 = motor::Module::new("Position");
    let module2 = motor::Module::new("Controls");
    let entity = motor::new_entity!("Entity");

    loop {
        clear_background(macroquad::color::PURPLE);
        if motor::input::is_key_pressed(motor::input::KeyCode::Escape) {
            println!("Esc pressed");
            std::process::exit(0)
        }
        next_frame().await
    }
}
