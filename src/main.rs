use macroquad::prelude::*;
use motor;

#[macroquad::main("Testing")]
async fn main() {
    println!("Hello, world!");

    let player = motor::PlayerEntity::new("Player", (10.0,10.0));

    loop {
        clear_background(macroquad::color::PURPLE);
        next_frame().await
    }
}
