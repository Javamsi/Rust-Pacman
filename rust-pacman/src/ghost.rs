extern crate graphics;
extern crate piston_window;

use piston_window::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::rc::Rc;
use std::cell::RefCell;
use game::Game;

pub struct Ghost {
	loc: (i32,i32),
	target: (i32, i32),
	name: String,
	state: i32, //0 for scatter, 1 for frightened, 2 for attack
	pac_loc: (i32, i32),
	coordinates: Vec<(i32, i32, i32, i32)>,
	intersections: Vec<(i32, i32)>,
	cur_direction: i32, // 1 for up, 2 for down, 3 for left, 4 for right
	directions: Vec<i32>,
}

impl Ghost {

	pub fn new(name: String, start_x: i32, start_y: i32) -> Ghost {

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

		let mut location = (start_x, start_y);

		let mut pac_loc = (0, 0);

		let mut directions: Vec<i32> = Vec::new();

		Ghost {
			loc: location,
			name: name,
			state: 2,
			pac_loc: pac_loc,
			target: (0,0),
			cur_direction: 4,
			coordinates: coordinates,
			intersections: intersections,
			directions: directions,
		}
	}

	pub fn update_pac_loc(&mut self, x: i32, y: i32) {
		self.pac_loc.0 = x;
		self.pac_loc.1 = y;
	}

	pub fn check_walls(&mut self) {
		for &(x,y,width,length) in &self.coordinates {
			if (width == 10) {
				if (y > y + length) {
					if (self.loc.1 >= y + length && self.loc.1 <= y 
						&& self.loc.0 + 25 >= x && self.loc.0 < x) {
						if (!self.directions.contains(&4)) {
							self.directions.push(4);
						}
					}
					else if (self.loc.1 >= y + length && self.loc.1 <= y 
						&& self.loc.0 - 25 <= x && self.loc.0 > x) {
						if (!self.directions.contains(&3)) {
							self.directions.push(3);
						}
					}
				}
				else if (y < y + length) {
					if (self.loc.1 >= y && self.loc.1 <= y + length
						&& self.loc.0 + 25 >= x && self.loc.0 < x) {
						if (!self.directions.contains(&4)) {
							self.directions.push(4);
						}
					}
					else if (self.loc.1 >= y && self.loc.1 <= y + length
						&& self.loc.0 - 25 <= x && self.loc.0 > x) {
						if (!self.directions.contains(&3)) {
							self.directions.push(3);
						}
					}
				}
			}
			else if (length == 10) {
				if (x > x + width) {
					if (self.loc.0 >= x + width && self.loc.0 <= x
						&& self.loc.1 + 25 >= y && self.loc.1 < y) {
						if (!self.directions.contains(&2)) {
							self.directions.push(2);
						}
					}
					else if (self.loc.0 >= x + width && self.loc.0 <= x 
						&& self.loc.1 - 25 <= y && self.loc.1 > y) {
						if (!self.directions.contains(&1)) {
							self.directions.push(1);
						}
					}
				}
				else if (x < x + width) {
					if (self.loc.0 >= x && self.loc.0 <= x + width
						&& self.loc.1 + 25 >= y && self.loc.1 < y) {
						if (!self.directions.contains(&2)) {
							self.directions.push(2);
						}
					}
					else if (self.loc.0 >= x && self.loc.0 <= x + width
						&& self.loc.1 - 25 <= y && self.loc.1 > y) {
						if (!self.directions.contains(&1)) {
							self.directions.push(1);
						}
					}
				}
			}
		}
	}

	pub fn is_intersection(&mut self) -> bool {
		for &(x,y) in &self.intersections {
			if (self.loc.0 == x && self.loc.1 == y) {
				return true
			}
		}
		return false
	}

	pub fn chase(&mut self) {

		if self.name == "blinky" {
			if self.state == 0 {
				
			}
			else if self.state == 1 {
				
			}
			else {
				self.target.0 = self.pac_loc.0;
				self.target.1 = self.pac_loc.1;
			}
		}
		else if self.name == "pinky" {
			//Pinky
		}
		else if self.name == "clyde" {
			//Clyde
		}
		else if self.name == "inky" {
			//Inky
		}

		if (self.is_intersection()) {
			self.check_walls();

			if (self.directions.len() == 3) {
				if(!self.directions.contains(&1)) {
					self.cur_direction = 1;
				}
				else if (!self.directions.contains(&2)) {
					self.cur_direction = 2;
				}
				else if (!self.directions.contains(&3)) {
					self.cur_direction = 3;
				}
				else if (!self.directions.contains(&4)) {
					self.cur_direction = 4;
				}
			}
			if (self.directions.len() == 2) {

			}
			if (self.directions.len() == 1) {

			}
		}
	}


}