extern crate graphics;
extern crate piston_window;

use piston_window::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use ghost::Ghost;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Game {
	window: PistonWindow,
	pac_loc: Vec<(i32, i32)>,
	coordinates: Vec<(i32, i32, i32, i32)>,
	intersections: Vec<(i32, i32)>,
	pac_up: bool,
	pac_down: bool,
	pac_right: bool,
	pac_left: bool,
	next_move: i32,
	blinky: Ghost,
	pinky: Ghost,
	inky: Ghost,
	clyde: Ghost,
	pac_direction: i32,
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

		file = BufReader::new(File::open("intersections.txt").unwrap());
		let mut intersections = Vec::<(i32, i32)>::with_capacity(2);

		// Parse the intersections file for intersections
		for (i, line) in file.lines().enumerate() {
			for (j, number) in line.unwrap().split(char::is_whitespace).enumerate() {
				let v: Vec<&str> = number.split(",").collect();
				intersections.push((v[0].parse::<i32>().unwrap(), v[1].parse::<i32>().unwrap()));
			}
		}

		let mut blinky: Ghost = Ghost::new(String::from("blinky"), 195, 475);
		let mut pinky: Ghost = Ghost::new(String::from("pinky"), 195, 475);
		let mut inky: Ghost = Ghost::new(String::from("inky"), 195, 475);
		let mut clyde: Ghost = Ghost::new(String::from("clyde"), 195, 475);

		Game {
			window: window,
			pac_loc: location,
			coordinates: coordinates,
			intersections: intersections,
			pac_up: false,
			pac_down: false,
			pac_right: false,
			pac_left: false,
			next_move: 0,
			blinky: blinky,
			pinky: pinky,
			inky: inky,
			clyde: clyde,
			pac_direction: 0,
		}
	}

	pub fn input(&mut self, inp: Input) {
		match inp {
			Input::Press(but) => {
				match but {
					Button::Keyboard(Key::Up) => {
						self.next_move = 1;
					},
					Button::Keyboard(Key::Down) => {
						self.next_move = 2;
					},
					Button::Keyboard(Key::Left) => {
						self.next_move = 3;
					},
					Button::Keyboard(Key::Right) => {
						self.next_move = 4;
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

		/* Draw the Intersections */
       	for &(x,y) in &self.intersections {
			self.window.draw_2d(&e, |c, g| {
           		rectangle([1.0, 0.0, 1.0, 1.0], 
                 	[(x - 5) as f64, (y - 5) as f64, 20.0, 20.0],
               	    c.transform, g);		         
       		});
		}

		/* Update PacMan */
		for &(x,y) in &self.pac_loc {
			self.window.draw_2d(&e, |c, g| {
           		rectangle([1.0, 1.0, 0.0, 1.0], 
                     [(x - 5) as f64, (y - 5) as f64, 20.0, 20.0],
                     c.transform, g);		         
       		});
		}

		let blinkyx: i32 = self.blinky.get_loc().0;
		let blinkyy: i32 = self.blinky.get_loc().1;
		let pinkyx: i32 = self.pinky.get_loc().0;
		let pinkyy: i32 = self.pinky.get_loc().1;
		let inkyx: i32 = self.inky.get_loc().0;
		let inkyy: i32 = self.inky.get_loc().1;
		let clydex: i32 = self.clyde.get_loc().0;
		let clydey: i32 = self.clyde.get_loc().1;

		/* Draw Blinky */
		self.window.draw_2d(&e, |c, g| {
           	rectangle([1.0, 0.0, 0.0, 1.0], 
            	[(blinkyx - 5) as f64, (blinkyy - 5) as f64, 20.0, 20.0],
            	c.transform, g);		         
       	});

       	/* Draw Pinky */
		self.window.draw_2d(&e, |c, g| {
           	rectangle([1.0, 0.0, 0.5, 1.0], 
            	[(pinkyx - 5) as f64, (pinkyy - 5) as f64, 20.0, 20.0],
            	c.transform, g);		         
       	});

       	/* Draw Pinky */
		self.window.draw_2d(&e, |c, g| {
           	rectangle([0.0, 0.7, 1.0, 1.0], 
            	[(inkyx - 5) as f64, (inkyy - 5) as f64, 20.0, 20.0],
            	c.transform, g);		         
       	});

       	/* Draw Clyde */
		self.window.draw_2d(&e, |c, g| {
           	rectangle([1.0, 0.5, 0.0, 1.0], 
            	[(clydex - 5) as f64, (clydey - 5) as f64, 20.0, 20.0],
            	c.transform, g);		         
       	});
	}

	pub fn check_collision(&mut self) {
		
		for &(x,y) in &self.intersections {
			if (self.pac_loc[0].0 == x && self.pac_loc[0].1 == y) {
				/* Get New direction */
				if (self.next_move == 1) {
					self.pac_up = true;
					self.pac_down = false;
					self.pac_left = false;
					self.pac_right = false;
				}
				else if (self.next_move == 2) {
					self.pac_down = true;
					self.pac_up = false;
					self.pac_right = false;
					self.pac_left = false;
				}
				else if (self.next_move == 3) {
					self.pac_left = true;
					self.pac_up = false;
					self.pac_right = false;
					self.pac_down = false;
				}
				else if (self.next_move == 4) {
					self.pac_right = true;
					self.pac_up = false;
					self.pac_down = false;
					self.pac_left = false;
				}
			}
		}

		/* Change to opposite direction if in the middle */
		if (self.next_move == 1 && self.pac_down) {
			self.pac_up = true;
			self.pac_down = false;
			self.next_move = 0;
		}
		else if (self.next_move == 2 && self.pac_up) {
			self.pac_down = true;
			self.pac_up = false;
			self.next_move = 0;
		}
		else if (self.next_move == 3 && self.pac_right) {
			self.pac_left = true;
			self.pac_right = false;
			self.next_move = 0;
		}
		else if (self.next_move == 4 && self.pac_left) {
			self.pac_right = true;
			self.pac_left = false;
			self.next_move = 0;
		}		

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
		self.pac_loc.push((455,605));

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
						self.pac_direction = 1;
					}
					else if self.pac_down {
						self.pac_loc[0].1 = self.pac_loc[0].1 + 5;
						self.pac_direction = 2;
					}
					else if self.pac_left {
						self.pac_loc[0].0 = self.pac_loc[0].0 - 5;
						self.pac_direction = 3;
					}
					else if self.pac_right {
						self.pac_loc[0].0 = self.pac_loc[0].0 + 5;
						self.pac_direction = 4;
					}
					self.blinky.update_pac_loc(self.pac_loc[0].0, self.pac_loc[0].1, self.pac_direction); 
					self.blinky.chase();
					self.pinky.update_pac_loc(self.pac_loc[0].0, self.pac_loc[0].1, self.pac_direction);
					self.pinky.chase();
					self.inky.update_pac_loc(self.pac_loc[0].0, self.pac_loc[0].1, self.pac_direction);
					self.inky.update_blinky_loc(self.blinky.get_loc().0, self.blinky.get_loc().1);
					self.inky.chase();
					self.clyde.update_pac_loc(self.pac_loc[0].0, self.pac_loc[0].1, self.pac_direction);
					self.clyde.chase();
				}
			}
			
		}
	}
}