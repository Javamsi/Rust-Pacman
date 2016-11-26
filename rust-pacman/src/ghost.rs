extern crate graphics;
extern crate piston_window;
extern crate rand;

use piston_window::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::rc::Rc;
use std::cell::RefCell;
use game::Game;
use rand::Rng;

pub struct Ghost {
	loc: (i32,i32),
	target: (i32, i32),
	name: String,
	state: i32, //0 for scatter, 1 for frightened, 2 for attack
	pac_loc: (i32, i32),
	blinky_loc: (i32, i32),
	pac_direction: i32,
	coordinates: Vec<(i32, i32, i32, i32)>,
	intersections: Vec<(i32, i32)>,
	cur_direction: i32, // 1 for up, 2 for down, 3 for left, 4 for right
	directions: Vec<i32>,
	time: i32,
	ghost_name: String,
	ghost_counter: i32,
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

		let mut sprite_name: String = String::new();

		/* Assign the associated sprite name */
		if (name == "blinky") {
			sprite_name = String::from("blinky_right.png");
		}
		else if (name == "pinky") {
			sprite_name = String::from("pinky_right.png");
		}
		else if (name == "inky") {
			sprite_name = String::from("inky_right.png");
		}
		else if (name == "clyde") {
			sprite_name = String::from("clyde_right.png");
		}

		Ghost {
			loc: location,
			name: name,
			state: 0,
			pac_loc: pac_loc,
			target: (0,0),
			cur_direction: 1,
			coordinates: coordinates,
			intersections: intersections,
			pac_direction: 0,
			blinky_loc: (0,0),
			directions: directions,
			time: 0,
			ghost_name: sprite_name,
			ghost_counter: 0,
		}
	}

	pub fn update_pac_loc(&mut self, x: i32, y: i32, direction: i32) {
		self.pac_loc.0 = x;
		self.pac_loc.1 = y;
		self.pac_direction = direction;
	}

	pub fn update_blinky_loc(&mut self, x: i32, y: i32) {
		self.blinky_loc.0 = x;
		self.blinky_loc.1 = y;
	}

	pub fn get_loc(&mut self) -> (i32,i32) { self.loc }

	pub fn check_walls(&mut self) {
		for &(x,y,width,length) in &self.coordinates {
			if width == 10 {
				if y > y + length {
					if self.loc.1 >= y + length && self.loc.1 <= y 
						&& self.loc.0 + 25 >= x && self.loc.0 < x {
						if !self.directions.contains(&4) {
							self.directions.push(4);
						}
					}
					else if self.loc.1 >= y + length && self.loc.1 <= y 
						&& self.loc.0 - 25 <= x && self.loc.0 > x {
						if !self.directions.contains(&3) {
							self.directions.push(3);
						}
					}
				}
				else if y < y + length {
					if self.loc.1 >= y && self.loc.1 <= y + length
						&& self.loc.0 + 25 >= x && self.loc.0 < x {
						if !self.directions.contains(&4) {
							self.directions.push(4);
						}
					}
					else if self.loc.1 >= y && self.loc.1 <= y + length
						&& self.loc.0 - 25 <= x && self.loc.0 > x {
						if !self.directions.contains(&3) {
							self.directions.push(3);
						}
					}
				}
			}
			else if length == 10 {
				if x > x + width {
					if self.loc.0 >= x + width && self.loc.0 <= x
						&& self.loc.1 + 25 >= y && self.loc.1 < y {
						if !self.directions.contains(&2) {
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
				else if x < x + width {
					if self.loc.0 >= x && self.loc.0 <= x + width
						&& self.loc.1 + 25 >= y && self.loc.1 < y {
						if !self.directions.contains(&2) {
							self.directions.push(2);
						}
					}
					else if self.loc.0 >= x && self.loc.0 <= x + width
						&& self.loc.1 - 25 <= y && self.loc.1 > y {
						if !self.directions.contains(&1) {
							self.directions.push(1);
						}
					}
				}
			}
		}
	}

	pub fn is_intersection(&mut self) -> bool {
		for &(x,y) in &self.intersections {
			if self.loc.0 == x && self.loc.1 == y {
				return true
			}
		}
		return false
	}

	pub fn get_sprite_name(&mut self) -> String {
		self.ghost_name.clone()
	}

	pub fn getTwoDistance(&mut self) {
		let mut upx: i32 = self.loc.0;
		let mut upy: i32 = self.loc.1 - 5;
		let mut downx: i32 = self.loc.0;
		let mut downy: i32 = self.loc.1 + 5;
		let mut leftx: i32 = self.loc.0 - 5;
		let mut lefty: i32 = self.loc.1;
		let mut rightx: i32 = self.loc.0 + 5;
		let mut righty: i32 = self.loc.1;
		let mut dir: i32 = 0;
		
		let mut up_dist: i32 = ((self.target.0 - upx) * (self.target.0 - upx))
						+ ((self.target.1 - upy) * (self.target.1 - upy));
		let mut down_dist: i32 = ((self.target.0 - downx) * (self.target.0 - downx))
						+ ((self.target.1 - downy) * (self.target.1 - downy));
		let mut left_dist: i32 = ((self.target.0 - leftx) * (self.target.0 - leftx))
						+ ((self.target.1 - lefty) * (self.target.1 - lefty));
		let mut right_dist: i32 = ((self.target.0 - rightx) * (self.target.0 - rightx))
						+ ((self.target.1 - righty) * (self.target.1 - righty));

		if self.cur_direction == 1 && self.directions.contains(&3) {
			if up_dist < right_dist { self.cur_direction = 1; }
			else { self.cur_direction = 4; }
		}
		else if self.cur_direction == 1 && self.directions.contains(&4) {
			if up_dist < left_dist { self.cur_direction = 1; }
			else { self.cur_direction = 3; }
		}
		else if self.cur_direction == 1 && self.directions.contains(&2) {
			if left_dist < right_dist { self.cur_direction = 3; }
			else { self.cur_direction = 4; }
		}
		else if self.cur_direction == 2 && self.directions.contains(&3) {
			if down_dist < right_dist { self.cur_direction = 2; }
			else { self.cur_direction = 4; }
		}
		else if self.cur_direction == 2 && self.directions.contains(&4) {
			if down_dist < left_dist { self.cur_direction = 2; }
			else { self.cur_direction = 3; }
		}
		else if self.cur_direction == 2 && self.directions.contains(&1) {
			if left_dist < right_dist { self.cur_direction = 3; }
			else { self.cur_direction = 4; }
		}
		else if self.cur_direction == 3 && self.directions.contains(&1) {
			if down_dist < left_dist { self.cur_direction = 2; }
			else { self.cur_direction = 3; }
		}	
		else if self.cur_direction == 3 && self.directions.contains(&2) {
			if up_dist < left_dist { self.cur_direction = 1; }
			else { self.cur_direction = 3; }
		}
		else if self.cur_direction == 3 && self.directions.contains(&4) {
			if up_dist < down_dist { self.cur_direction = 1; }
			else { self.cur_direction = 2; }
		}
		else if self.cur_direction == 4 && self.directions.contains(&1) {
			if down_dist < right_dist { self.cur_direction = 2; }
			else { self.cur_direction = 4; }
		}
		else if self.cur_direction == 4 && self.directions.contains(&2) {
			if up_dist < right_dist { self.cur_direction = 1; }
			else { self.cur_direction = 4; }
		}	
		else if self.cur_direction == 4 && self.directions.contains(&3) {
			if up_dist < down_dist { self.cur_direction = 1; }
			else { self.cur_direction = 2; }
		}						
	}

	pub fn getFourDistance(&mut self) {
		let mut upx: i32 = self.loc.0;
		let mut upy: i32 = self.loc.1 - 5;
		let mut downx: i32 = self.loc.0;
		let mut downy: i32 = self.loc.1 + 5;
		let mut leftx: i32 = self.loc.0 - 5;
		let mut lefty: i32 = self.loc.1;
		let mut rightx: i32 = self.loc.0 + 5;
		let mut righty: i32 = self.loc.1;
		let mut dir: i32 = 0;
		
		let mut up_dist: i32 = ((self.target.0 - upx) * (self.target.0 - upx))
						+ ((self.target.1 - upy) * (self.target.1 - upy));
		let mut down_dist: i32 = ((self.target.0 - downx) * (self.target.0 - downx))
						+ ((self.target.1 - downy) * (self.target.1 - downy));
		let mut left_dist: i32 = ((self.target.0 - leftx) * (self.target.0 - leftx))
						+ ((self.target.1 - lefty) * (self.target.1 - lefty));
		let mut right_dist: i32 = ((self.target.0 - rightx) * (self.target.0 - rightx))
						+ ((self.target.1 - righty) * (self.target.1 - righty));

		if self.cur_direction == 1 {
			if up_dist <= right_dist && up_dist <= left_dist { self.cur_direction = 1; }
			else if left_dist < right_dist && left_dist < up_dist { self.cur_direction = 3; }
			else { self.cur_direction = 4; }
		}
		else if self.cur_direction == 2 {
			if down_dist <= right_dist && down_dist <= left_dist { self.cur_direction = 2; }
			else if left_dist < right_dist && left_dist < down_dist { self.cur_direction = 3; }
			else { self.cur_direction = 4; }
		}
		else if self.cur_direction == 4 {
			if down_dist <= up_dist && down_dist <= right_dist { self.cur_direction = 2; }
			else if right_dist <= up_dist && right_dist <= down_dist { self.cur_direction = 4; }
			else { self.cur_direction = 1; }
		}
		else if self.cur_direction == 3 {
			if down_dist <= left_dist && down_dist <= up_dist { self.cur_direction = 2; }
			else if up_dist < left_dist &&  up_dist < down_dist { self.cur_direction = 1; }
			else { self.cur_direction = 3; }
		}
	}

	pub fn chase(&mut self) {

		/* Alternate between Scatter and Attack Modes */
		if (self.ghost_counter == 700) {
			self.state = 2;
		}
		else if (self.ghost_counter == 2700) {
			self.state = 0;
			self.ghost_counter = 0;
		}

		/* Ghost Logic for each Mode */
		if self.name == "blinky" {
			if self.state == 0 {
				self.target.0 = 0;
				self.target.1 = 0;
			}
			else if self.state == 1 {
				
			}
			else {
				self.target.0 = self.pac_loc.0;
				self.target.1 = self.pac_loc.1;
			}
		}
		else if self.name == "pinky" {
			if self.state == 0 {
				self.target.0 = 900;
				self.target.1 = 0;
			}
			else if self.state == 1 {

			}
			else {
				if self.pac_direction == 1 {
					self.target.0 = self.pac_loc.0;
					self.target.1 = self.pac_loc.1 - 20;
				}
				else if self.pac_direction == 2 {
					self.target.0 = self.pac_loc.0;
					self.target.1 = self.pac_loc.1 + 20;
				}
				else if self.pac_direction == 3 {
					self.target.0 = self.pac_loc.0 - 20;
					self.target.1 = self.pac_loc.1;
				}
				else if self.pac_direction == 4 {
					self.target.0 == self.pac_loc.0 + 20;
					self.target.1 = self.pac_loc.1;
				}
			}
		}
		else if self.name == "clyde" {
			if self.state == 0 {
				self.target.0 = 900;
				self.target.1 = 900;
			}
			else if self.state == 1 {

			}
			else {
				let mut sq_distancex = (self.loc.0 - self.pac_loc.0) * (self.loc.0 - self.pac_loc.0);
				let mut sq_distancey = (self.loc.0 - self.pac_loc.0) * (self.loc.0 - self.pac_loc.0);
				if (sq_distancex + sq_distancey) < 25600 {
					self.target.0 = 0;
					self.target.1 = 0;
				}
				else {
					self.target.0 = self.pac_loc.0;
					self.target.1 = self.pac_loc.1;
				}
			}
		}
		else if self.name == "inky" {
			if self.state == 0 {
				self.target.0 = 0;
				self.target.1 = 900;
			}
			else if self.state == 1 {

			}
			else {
				let mut offsetx: i32 = 0;
				let mut offsety: i32 = 0;

				if self.pac_direction == 1 {
					offsetx = self.pac_loc.0;
					offsety = self.pac_loc.1 - 10;
				}
				else if self.pac_direction == 2 {
					offsetx = self.pac_loc.0;
					offsety = self.pac_loc.1 + 10;
				}
				else if self.pac_direction == 3 {
					offsetx = self.pac_loc.0 - 10;
					offsety = self.pac_loc.1;
				}
				else if self.pac_direction == 4 {
					offsetx == self.pac_loc.0 + 10;
					offsety = self.pac_loc.1;
				}

				self.target.0 = self.pac_loc.0 + (offsetx - self.blinky_loc.0);
				self.target.1 = self.pac_loc.1 + (offsety - self.blinky_loc.1);
			}
		}

		// Add the opposite direction 
		if self.cur_direction == 1 { self.directions.push(2); }
		else if self.cur_direction == 2 {self.directions.push(1); }
		else if self.cur_direction == 3 {self.directions.push(4); }
		else if self.cur_direction == 4 {self.directions.push(3); }

		/* Check for Intersection */
		if self.is_intersection() {

			self.check_walls();

			if self.directions.len() == 3 {
				if !self.directions.contains(&1) {
					self.cur_direction = 1;
				}
				else if !self.directions.contains(&2) {
					self.cur_direction = 2;
				}
				else if !self.directions.contains(&3) {
					self.cur_direction = 3;
				}
				else if !self.directions.contains(&4) {
					self.cur_direction = 4;
				}
			}
			else if self.directions.len() == 2 {
				self.getTwoDistance();
			}
			else if self.directions.len() == 1 {
				self.getFourDistance();
			}
		}

		/* Update Location */
		if self.cur_direction == 1 { self.loc.1 = self.loc.1 - 1; }
		if self.cur_direction == 2 { self.loc.1 = self.loc.1 + 1; }
		if self.cur_direction == 3 { self.loc.0 = self.loc.0 - 1; }
		if self.cur_direction == 4 { self.loc.0 = self.loc.0 + 1; }

		/* Assign the associated sprite image */
		if self.name == "blinky" && self.cur_direction == 1 { self.ghost_name = String::from("blinky_up.png"); }
		else if self.name == "blinky" && self.cur_direction == 2 { self.ghost_name = String::from("blinky_down.png"); }
		else if self.name == "blinky" && self.cur_direction == 3 { self.ghost_name = String::from("blinky_left.png"); }
		else if (self.name == "blinky" && self.cur_direction == 4) { self.ghost_name = String::from("blinky_right.png"); }
		
		if self.name == "pinky" && self.cur_direction == 1 { self.ghost_name = String::from("pinky_up.png");}
		else if self.name == "pinky" && self.cur_direction == 2 { self.ghost_name = String::from("pinky_down.png");}
		else if self.name == "pinky" && self.cur_direction == 3 { self.ghost_name = String::from("pinky_left.png");}
		else if self.name == "pinky" && self.cur_direction == 4 { self.ghost_name = String::from("pinky_right.png");}
		
		if self.name == "inky" && self.cur_direction == 1 { self.ghost_name = String::from("inky_up.png"); }
		else if self.name == "inky" && self.cur_direction == 2 { self.ghost_name = String::from("inky_down.png"); }
		else if self.name == "inky" && self.cur_direction == 3 { self.ghost_name = String::from("inky_left.png"); }
		else if self.name == "inky" && self.cur_direction == 4 { self.ghost_name = String::from("inky_right.png"); }
		
		if self.name == "clyde" && self.cur_direction == 1 { self.ghost_name = String::from("clyde_up.png"); }
		else if self.name == "clyde" && self.cur_direction == 2 { self.ghost_name = String::from("clyde_down.png"); }
		else if self.name == "clyde" && self.cur_direction == 3 { self.ghost_name = String::from("clyde_left.png"); }
		else if self.name == "clyde" && self.cur_direction == 4 { self.ghost_name = String::from("clyde_right.png"); }

		self.directions.clear();

		/* Increment ghost counter */
		if (self.state != 1) {
			self.ghost_counter = self.ghost_counter + 1;
		}
	} 


}