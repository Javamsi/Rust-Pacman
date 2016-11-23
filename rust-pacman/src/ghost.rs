extern crate graphics;
extern crate piston_window;

use piston_window::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Ghost {
	currentLocation: Vec<(i32,i32)>,
	name: String,
}

impl Ghost {

	pub fn new() -> Ghost {

		let mut location = Vec::<(i32, i32)>::with_capacity(2);

		location.push(30,30);
		let mut name = "abc";

		Ghost {
			currentLocation: location,
			name: name,
		}
	}

	pub fn chase() {
		if (self.name = "blinky") {

		}
		else if (self.name == "pinky") {

		}
		else if (self.name == "clyde") {

		}
		else if (self.name == "inky") {

		}
	}


}