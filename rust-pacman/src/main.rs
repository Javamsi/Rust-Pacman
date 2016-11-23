extern crate piston_window;
extern crate graphics;

mod game;
mod ghost;

fn main() {
	game::Game::new().run();
}