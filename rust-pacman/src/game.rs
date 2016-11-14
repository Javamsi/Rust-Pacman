extern crate graphics;
extern crate piston_window;

use piston_window::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Game {
	window: PistonWindow,
	pac_loc: Vec<(i32, i32)>,
	coordinates: Vec<(i32, i32, i32, i32)>,
	pac_up: bool,
	pac_down: bool,
	pac_right: bool,
	pac_left: bool,
}

impl Game {

	pub fn new() -> Game  {
		let mut window: PistonWindow = WindowSettings::new ("Rust PacMan", (930, 1000))
		.exit_on_esc(true)
		.build()
		.unwrap();

		let mut location = Vec::<(i32, i32)>::with_capacity(2);

		let mut file = BufReader::new(File::open("squarelocation.txt").unwrap());
		let mut coordinates = Vec::<(i32, i32, i32, i32)>::with_capacity(4);

		// Parse the coordinates file for the coordinates
		for (i, line) in file.lines().enumerate() {
			for (j, number) in line.unwrap().split(char::is_whitespace).enumerate() {
				let v: Vec<&str> = number.split(",").collect();
				coordinates.push((v[0].parse::<i32>().unwrap(), v[1].parse::<i32>().unwrap(), v[2].parse::<i32>().unwrap(), v[3].parse::<i32>().unwrap()));
			}
		}

		Game {
			window: window,
			pac_loc: location,
			coordinates: coordinates,
			pac_up: false,
			pac_down: false,
			pac_right: false,
			pac_left: false,
		}
	}

	pub fn input(&mut self, inp: Input) {
		match inp {
			Input::Press(but) => {
				match but {
					Button::Keyboard(Key::Up) => {
						self.pac_up = true;
						self.pac_right = false;
						self.pac_down = false;
						self.pac_left = false;
					},
					Button::Keyboard(Key::Down) => {
						self.pac_down = true;
						self.pac_up = false;
						self.pac_right = false;
						self.pac_left = false;
					},
					Button::Keyboard(Key::Left) => {
						self.pac_left = true;
						self.pac_right = false;
						self.pac_down = false;
						self.pac_up = false;
					},
					Button::Keyboard(Key::Right) => {
						self.pac_right = true;
						self.pac_up = false;
						self.pac_left = false;
						self.pac_down = false;
					}, 
					_ => {}
				}
			},
			_ => {}
		}
	}

	pub fn draw(&mut self, ren: RenderArgs, e: Event) {

		self.window.draw_2d(&e, |c, g| {
           	clear([0.0, 0.0, 0.0, 1.0], g)         
        });
		/* Draw the Game Board */
       	for &(x,y,width,length) in &self.coordinates {
			self.window.draw_2d(&e, |c, g| {
           		rectangle([0.0, 0.0, 10.0, 1.0], 
                 	[x as f64, y as f64, width as f64, length as f64],
               	    c.transform, g);		         
       		});
		}

		/* Update PacMan */
		for &(x,y) in &self.pac_loc {
			self.window.draw_2d(&e, |c, g| {
           		rectangle([1.0, 1.0, 0.0, 1.0], 
                     [x as f64, y as f64, 20.0, 20.0],
                     c.transform, g);		         
       		});
		}

	}

	pub fn check_collision(&mut self) {
		
		for &(x,y,width,length) in &self.coordinates {
			if (width == 10) {
				if (y > y + length) {
					if (self.pac_loc[0].1 >= y + length && self.pac_loc[0].1 <= y 
						&& self.pac_loc[0].0 + 25 >= x && self.pac_loc[0].0 < x && self.pac_right) {
						self.pac_right = false;
					}
					else if (self.pac_loc[0].1 >= y + length && self.pac_loc[0].1 <= y 
						&& self.pac_loc[0].0 - 25 <= x && self.pac_loc[0].0 > x && self.pac_left) {
						self.pac_left = false;
					}
				}
				else if (y < y + length) {
					if (self.pac_loc[0].1 >= y && self.pac_loc[0].1 <= y + length
						&& self.pac_loc[0].0 + 25 >= x && self.pac_loc[0].0 < x && self.pac_right) {
						self.pac_right = false;
					}
					else if (self.pac_loc[0].1 >= y && self.pac_loc[0].1 <= y + length
						&& self.pac_loc[0].0 - 25 <= x && self.pac_loc[0].0 > x && self.pac_left) {
						self.pac_left = false;
					}
				}
			}
			else if (length == 10) {
				if (x > x + width) {
					if (self.pac_loc[0].0 >= x + width && self.pac_loc[0].0 <= x
						&& self.pac_loc[0].1 + 25 >= y && self.pac_loc[0].1 < y && self.pac_down) {
						self.pac_down = false;
					}
					else if (self.pac_loc[0].0 >= x + width && self.pac_loc[0].0 <= x 
						&& self.pac_loc[0].1 - 25 <= y && self.pac_loc[0].1 > y && self.pac_up) {
						self.pac_up = false;
					}
				}
				else if (x < x + width) {
					if (self.pac_loc[0].0 >= x && self.pac_loc[0].0 <= x + width
						&& self.pac_loc[0].1 + 25 >= y && self.pac_loc[0].1 < y && self.pac_down) {
						self.pac_down = false;
					}
					else if (self.pac_loc[0].0 >= x && self.pac_loc[0].0 <= x + width
						&& self.pac_loc[0].1 - 25 <= y && self.pac_loc[0].1 > y && self.pac_up) {
						self.pac_up = false;
					}
				}
			}
		} 
	}

	pub fn run(&mut self) {

		/* Initialize Pac Man location */
		self.pac_loc.push((450,600));

		while let Some(e) = self.window.next() {

			/* Match the event */
			match e {
				Event::Input(inp) => {
					self.input(inp);
				},
				Event::Render(ren) => {
					self.draw(ren, e);
				},
				Event::Update(upd) => {

				},
				_ => {
					self.check_collision();
					if self.pac_up {
						self.pac_loc[0].1 = self.pac_loc[0].1 - 5;
					}
					if self.pac_down {
						self.pac_loc[0].1 = self.pac_loc[0].1 + 5;
					}
					if self.pac_left {
						self.pac_loc[0].0 = self.pac_loc[0].0 - 5;
					}
					if self.pac_right {
						self.pac_loc[0].0 = self.pac_loc[0].0 + 5;
					}
				}
			}
			
		}
	}
}