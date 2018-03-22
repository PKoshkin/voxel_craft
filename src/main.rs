#[macro_use]
extern crate glium;
mod game_application;
use game_application::GameApplication;


fn main() {
    let mut game = GameApplication::new("shaders");
    game.start_loop();
}
