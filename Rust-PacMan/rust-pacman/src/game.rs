extern crate graphics;
extern crate piston_window;

use piston_window::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Game {
	window: PistonWindow,
}

impl Game {

	pub fn new() -> Game  {
		let mut window: PistonWindow = WindowSettings::new ("Rust PacMan", (1000, 1000))
		.exit_on_esc(true)
		.build()
		.unwrap();

		Game {
			window: window,
		}
	}

	pub fn run(&mut self) {

		let mut file = BufReader::new(File::open("squarelocation.txt").unwrap());
		let mut coordinates = Vec::<(i32, i32)>::with_capacity(2);

		// Parse the coordinates file for the coordinates
		for (i, line) in file.lines().enumerate() {
			for (j, number) in line.unwrap().split(char::is_whitespace).enumerate() {
				let v: Vec<&str> = number.split(",").collect();
				coordinates.push((v[0].parse::<i32>().unwrap(), v[1].parse::<i32>().unwrap()));
			}
		}

		while let Some(e) = self.window.next() {
			self.window.draw_2d(&e, |c, g| {
            	clear([1.0; 4], g);         
        	});

        	for &(x,y) in &coordinates {
				self.window.draw_2d(&e, |c, g| {
            		rectangle([0.0, 0.0, 10.0, 1.0], 
                      [x as f64, y as f64, 10.0, 10.0],
                      c.transform, g);		         
        		});

			}
			
		}
	}
}