#[macro_use]
extern crate glium;

mod game_application;
use game_application::GameApplication;


use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        let mut game = GameApplication::new(".");
        game.start_loop();
    } else if args.len() == 2 {
        let mut game = GameApplication::new(&args[1]);
        game.start_loop();
    } else {
        panic!("Wrong arguments number");
    }
}
