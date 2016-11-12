extern crate piston_window;
extern crate graphics;

mod game;

fn main() {
	game::Game::new().run();
}