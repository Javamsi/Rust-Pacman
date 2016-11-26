extern crate piston_window;
extern crate graphics;
extern crate find_folder;
extern crate gfx_device_gl;
extern crate gfx_graphics;
extern crate gfx;

mod game;
mod ghost;

fn main() {
	game::Game::new().run();
}