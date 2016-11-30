extern crate graphics;
extern crate piston_window;
extern crate find_folder;
extern crate gfx_device_gl;
extern crate gfx_graphics;
extern crate gfx;

use piston_window::*;
use gfx_device_gl::{Resources};
use std::fs::File;
use std::io::{BufRead, BufReader};
use ghost::Ghost;

pub struct Game {
	window: PistonWindow,
	pac_loc: Vec<(i32, i32)>,
	coordinates: Vec<(i32, i32, i32, i32)>,
	intersections: Vec<(i32, i32)>,
	pellets: Vec<(i32, i32)>,
	pac_up: bool,
	pac_down: bool,
	pac_right: bool,
	pac_left: bool,
	pac_direction: i32,
	next_move: i32,
	blinky: Ghost,
	pinky: Ghost,
	inky: Ghost,
	clyde: Ghost,
	pacman_sprite: Option<Texture<Resources>>,
	blinky_sprite: Option<Texture<Resources>>,
	pinky_sprite: Option<Texture<Resources>>,
	inky_sprite: Option<Texture<Resources>>,
	clyde_sprite: Option<Texture<Resources>>,
	pac_counter: i32,
	pac_name: String,
	game_score: i32,
	glyphs: Option<Glyphs>,
	normal_text: Option<Glyphs>,
	lives: i32,
}

impl Game {

	pub fn new() -> Game  {
		let opengl = OpenGL::V3_2;
		let mut window: PistonWindow = WindowSettings::new ("Rust PacMan", (930, 1000))
		.opengl(opengl)
		.exit_on_esc(true)
		.build()
		.unwrap();

		let location = Vec::<(i32, i32)>::with_capacity(2);

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

		file = BufReader::new(File::open("pellets.txt").unwrap());
		let mut pellets = Vec::<(i32, i32)>::with_capacity(2);

		// Parse the pellets file for the pellets
		for (i, line) in file.lines().enumerate() {
			for (j, number) in line.unwrap().split(char::is_whitespace).enumerate() {
				let v: Vec<&str> = number.split(",").collect();
				pellets.push((v[0].parse::<i32>().unwrap(), v[1].parse::<i32>().unwrap()));
			}
		}

		/* Initialize ghost location */
		let blinky: Ghost = Ghost::new(String::from("blinky"), 365, 435);
		let pinky: Ghost = Ghost::new(String::from("pinky"), 425, 435);
		let inky: Ghost = Ghost::new(String::from("inky"), 485, 435);
		let clyde: Ghost = Ghost::new(String::from("clyde"), 545, 435);

		Game {
			window: window,
			pac_loc: location,
			coordinates: coordinates,
			intersections: intersections,
			pellets: pellets,
			pac_up: false,
			pac_down: false,
			pac_right: false,
			pac_left: false,
			next_move: 0,
			blinky: blinky,
			pinky: pinky,
			inky: inky,
			clyde: clyde,
			pac_direction: 4,
			pacman_sprite: None,
			blinky_sprite: None,
			pinky_sprite: None,
			inky_sprite: None,
			clyde_sprite: None,
			pac_counter: 0,
			pac_name: String::from("pac_right.png"),
			game_score: 0,
			glyphs: None,
			normal_text: None,
			lives: 3,
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

	pub fn init_sprites(&mut self) {
		let assets = find_folder::Search::ParentsThenKids(3,3).for_folder("images").unwrap();

		/* Pac Man Sprite */
		let pacman_sprite = assets.join(self.pac_name.clone());
		let pacman_sprite = Texture::from_path(
			&mut self.window.factory,
			&pacman_sprite,
			Flip::None,
			&TextureSettings::new())
			.unwrap();
		self.pacman_sprite = Some(pacman_sprite);

		/* Blinky Sprite */
		let blinky_sprite = assets.join(self.blinky.get_sprite_name());
		let blinky_sprite = Texture::from_path(
			&mut self.window.factory,
			&blinky_sprite,
			Flip::None,
			&TextureSettings::new())
			.unwrap();
		self.blinky_sprite = Some(blinky_sprite);

		/* Pinky Sprite */
		let pinky_sprite = assets.join(self.pinky.get_sprite_name());
		let pinky_sprite = Texture::from_path(
			&mut self.window.factory,
			&pinky_sprite,
			Flip::None,
			&TextureSettings::new())
			.unwrap();
		self.pinky_sprite = Some(pinky_sprite);

		/* Inky Sprite */
		let inky_sprite = assets.join(self.inky.get_sprite_name());
		let inky_sprite = Texture::from_path(
			&mut self.window.factory,
			&inky_sprite,
			Flip::None,
			&TextureSettings::new())
			.unwrap();
		self.inky_sprite = Some(inky_sprite);

		/* Clyde Sprite */
		let clyde_sprite = assets.join(self.clyde.get_sprite_name());
		let clyde_sprite = Texture::from_path(
			&mut self.window.factory,
			&clyde_sprite,
			Flip::None,
			&TextureSettings::new())
			.unwrap(); 
		self.clyde_sprite = Some(clyde_sprite);


		/* Text */
		let text_assets = find_folder::Search::ParentsThenKids(3,3)
						.for_folder("images").unwrap();
		let ref font = text_assets.join("PAC-FONT.TTF");
		let factory = self.window.factory.clone();
		let mut glyphs = Glyphs::new(font, factory).unwrap();
		self.glyphs = Some(glyphs);

		/* Normal Text */
		let normal_assets = find_folder::Search::ParentsThenKids(3,3)
						.for_folder("images").unwrap();
		let ref normal_font = text_assets.join("emulogic.ttf");
		let normal_factory = self.window.factory.clone();
		let mut normal_glyphs = Glyphs::new(normal_font, normal_factory).unwrap();
		self.normal_text = Some(normal_glyphs);

	}

	pub fn draw(&mut self, ren: RenderArgs, e: Event) {

		let mut pellets: Vec<(i32,i32)> = Vec::new();
		let mut coordinates: Vec<(i32, i32, i32, i32)> = Vec::new();
		let mut pac_loc: Vec<(i32, i32)> = Vec::new();
		let mut score: String = self.game_score.to_string(); 
		let lives: i32 = self.lives;

		/* Ghost Locations */
		let blinkyx: i32 = self.blinky.get_loc().0;
		let blinkyy: i32 = self.blinky.get_loc().1;
		let pinkyx: i32 = self.pinky.get_loc().0;
		let pinkyy: i32 = self.pinky.get_loc().1;
		let inkyx: i32 = self.inky.get_loc().0;
		let inkyy: i32 = self.inky.get_loc().1;
		let clydex: i32 = self.clyde.get_loc().0;
		let clydey: i32 = self.clyde.get_loc().1;

		if self.pac_right && self.pac_counter == 0 {
			self.pac_name = String::from("pac_right_full.png");
			self.pac_counter = self.pac_counter + 1;
		}
		else if self.pac_right && self.pac_counter == 1 {
			self.pac_name = String::from("pac_right.png");
			self.pac_counter = self.pac_counter + 1;
		}
		else {
			self.pac_name = String::from("pac_close.png");
			self.pac_counter = 0;
		}

		/* Pac Man Location*/
		for &(x,y) in &self.pac_loc { pac_loc.push((x,y)); }

		/* Pellets */
		for &(x,y) in &self.pellets { pellets.push((x,y)); }

		/* Walls */
		for &(x,y,width,length) in &self.coordinates { coordinates.push((x,y,width,length)); }

		/* Draw Everything */

		/* Draw the Game Board */
		self.window.draw_2d(&e, |c, g| {
           	clear([0.0, 0.0, 0.0, 1.0], g); 

        	for (x,y,width,length) in coordinates {
           		rectangle([0.0, 0.0, 10.0, 1.0], 
                 	[x as f64, y as f64, width as f64, length as f64],
               	    c.transform, g);
           	}     	
        });

		/* Draw the Pellets */
        self.window.draw_2d(&e, |c, g| {
           	for (x,y) in pellets {
          		rectangle([1.0, 1.0, 0.0, 1.0], 
                     [(x - 3) as f64, (y - 3) as f64, 15.0, 15.0],
                     c.transform, g);	           		
           	}
        });

        /* Draw the Title, Score, Game Over */
        match self.glyphs {
        	None => {},
        	Some (ref mut glyph) => {
				self.window.draw_2d(&e, |c, g| {
					let transform = c.transform.trans(780.0, 340.0);
		   			text::Text::new_color([1.0, 1.0, 0.0, 1.0], 16).draw(
            		&"SCORE:",
            		glyph,
            		&c.draw_state,
            		transform, g );
        		});
				self.window.draw_2d(&e, |c, g| {
					let transform = c.transform.trans(390.0, 500.0);
		   			text::Text::new_color([1.0, 1.0, 0.0, 1.0], 16).draw(
            		&"PAC-MAN",
            		glyph,
            		&c.draw_state,
            		transform, g );
        		});
        		if (self.lives < 1) {
        			self.window.draw_2d(&e, |c, g| {
					let transform = c.transform.trans(370.0, 230.0);
		   			text::Text::new_color([1.0, 1.0, 0.0, 1.0], 16).draw(
            		&"GAME OVER",
            		glyph,
            		&c.draw_state,
            		transform, g );
        		});
        		}
        	}
        }

        match self.normal_text {
        	None => {},
        	Some (ref mut glyph) => {
        		self.window.draw_2d(&e, |c, g| {
					let transform = c.transform.trans(780.0, 380.0);
		   			text::Text::new_color([1.0, 1.0, 0.0, 1.0], 16).draw(
            		&score,
            		glyph,
            		&c.draw_state,
            		transform, g );
        		});
        	}
        }
        
        /* Draw the Lives */
        match self.glyphs {
        	None => {},
        	Some (ref mut glyph) => {
				self.window.draw_2d(&e, |c, g| {
					let transform = c.transform.trans(40.0, 340.0);
		   			text::Text::new_color([1.0, 1.0, 0.0, 1.0], 16).draw(
            		&"LIVES:",
            		glyph,
            		&c.draw_state,
            		transform, g );
        		});

        	}
        }

        /* Draw Pac Man */
        match self.pacman_sprite {
           	None => {}
           	Some(ref sprite) => {
           		self.window.draw_2d(&e, |c, g| {
           		let center = c.transform.trans((pac_loc[0].0 - 5)as f64, (pac_loc[0].1 - 5) as f64);
           				image(sprite, center, g);
           		});

           		/* Draw lives */
           		for i in 0..lives {
        			self.window.draw_2d(&e, |c, g| {
           				let center = c.transform.trans(40.0 + ((i as f64) * 30.0), 370.0 as f64);
           					image(sprite, center, g);
           			});
        		}
           	}
        }

        /* Draw Blinky */
        match self.blinky_sprite {
        	None => {}
       		Some(ref sprite) => {
       			self.window.draw_2d(&e, |c, g| {
       			let blinky_center = c.transform.trans((blinkyx - 5)as f64, (blinkyy - 5) as f64);
        		image(sprite, blinky_center, g);
        		});
         	}
        } 

        /* Draw Pinky */
        match self.pinky_sprite {
        	None => {}
       		Some(ref sprite) => {
       			self.window.draw_2d(&e, |c, g| {
       			let pinky_center = c.transform.trans((pinkyx - 5)as f64, (pinkyy - 5) as f64);
        		image(sprite, pinky_center, g);
        		});
         	}
        }  

        /* Draw Inky */
        match self.inky_sprite {
        	None => {}
       		Some(ref sprite) => {
       			self.window.draw_2d(&e, |c, g| {
       			let inky_center = c.transform.trans((inkyx - 5)as f64, (inkyy - 5) as f64);
        		image(sprite, inky_center, g);
        		});
         	}
        }  

        /* Draw Clyde */
        match self.clyde_sprite {
        	None => {}
       		Some(ref sprite) => {
       			self.window.draw_2d(&e, |c, g| {
       			let clyde_center = c.transform.trans((clydex - 5)as f64, (clydey - 5) as f64);
        		image(sprite, clyde_center, g);
        		});
         	}
        }          	

		/* Draw the Intersections */
       	/*for &(x,y) in &self.intersections {
			self.window.draw_2d(&e, |c, g| {
           		rectangle([1.0, 0.0, 1.0, 1.0], 
                 	[(x - 5) as f64, (y - 5) as f64, 20.0, 20.0],
               	    c.transform, g);		         
       		});
		}*/

	}

	pub fn update_lives(&mut self) {

		// Decrease number of lives
		self.lives = self.lives - 1;

		// Reset Ghost and Pacman Locations
		self.pac_loc[0].0 = 455;
		self.pac_loc[0].1 = 605;

		self.blinky.set_loc(365,435);
		self.pinky.set_loc(425,435);
		self.inky.set_loc(485,435);
		self.clyde.set_loc(545,435);
	}

	pub fn check_pellet(&mut self) {
		let mut counter: usize = 0;
		for &(x,y) in &self.pellets {
			if self.pac_loc[0].0 == x && self.pac_loc[0].1 == y {
				break;
			}
			counter = counter + 1;
		}
		if counter < self.pellets.len() {
			self.game_score = self.game_score + 100;
			self.pellets.remove(counter); 
		}
	}
	pub fn check_ghost_collision(&mut self) -> bool {

		let mut ghost_locations: Vec<(i32, i32)> = Vec::<(i32, i32)>::with_capacity(2);
		ghost_locations.push(self.blinky.get_loc());
		ghost_locations.push(self.pinky.get_loc());
		ghost_locations.push(self.inky.get_loc());
		ghost_locations.push(self.clyde.get_loc());

		for &(x,y) in &ghost_locations {
			if self.pac_loc[0].0 - 5 == x + 5 && self.pac_loc[0].1 == y { return true; }
			else if self.pac_loc[0].0 + 5 == x - 5 && self.pac_loc[0].1 == y { return true; }
			else if self.pac_loc[0].0 == x && self.pac_loc[0].1 + 5 == y - 5 { return true; }
			else if self.pac_loc[0].0 == x && self.pac_loc[0].1 - 5 == y + 5 { return true; }
		}

		return false;
	}

	pub fn check_collision(&mut self) {
		
		for &(x,y) in &self.intersections {
			if self.pac_loc[0].0 == x && self.pac_loc[0].1 == y {
				/* Get New direction */
				if self.next_move == 1 {
					self.pac_up = true;
					self.pac_down = false;
					self.pac_left = false;
					self.pac_right = false;
				}
				else if self.next_move == 2 {
					self.pac_down = true;
					self.pac_up = false;
					self.pac_right = false;
					self.pac_left = false;
				}
				else if self.next_move == 3 {
					self.pac_left = true;
					self.pac_up = false;
					self.pac_right = false;
					self.pac_down = false;
				}
				else if self.next_move == 4 {
					self.pac_right = true;
					self.pac_up = false;
					self.pac_down = false;
					self.pac_left = false;
				}
			}
		}

		/* Change to opposite direction if in the middle */
		if self.next_move == 1 && self.pac_down {
			self.pac_up = true;
			self.pac_down = false;
			self.next_move = 0;
		}
		else if self.next_move == 2 && self.pac_up {
			self.pac_down = true;
			self.pac_up = false;
			self.next_move = 0;
		}
		else if self.next_move == 3 && self.pac_right {
			self.pac_left = true;
			self.pac_right = false;
			self.next_move = 0;
		}
		else if self.next_move == 4 && self.pac_left {
			self.pac_right = true;
			self.pac_left = false;
			self.next_move = 0;
		}		

		for &(x,y,width,length) in &self.coordinates {
			if width == 10 {
				if y > y + length {
					if self.pac_loc[0].1 >= y + length && self.pac_loc[0].1 <= y 
						&& self.pac_loc[0].0 + 25 >= x && self.pac_loc[0].0 < x && self.pac_right {
						self.pac_right = false;
					}
					else if self.pac_loc[0].1 >= y + length && self.pac_loc[0].1 <= y 
						&& self.pac_loc[0].0 - 25 <= x && self.pac_loc[0].0 > x && self.pac_left {
						self.pac_left = false;
					}
				}
				else if y < y + length {
					if self.pac_loc[0].1 >= y && self.pac_loc[0].1 <= y + length
						&& self.pac_loc[0].0 + 25 >= x && self.pac_loc[0].0 < x && self.pac_right {
						self.pac_right = false;
					}
					else if self.pac_loc[0].1 >= y && self.pac_loc[0].1 <= y + length
						&& self.pac_loc[0].0 - 25 <= x && self.pac_loc[0].0 > x && self.pac_left {
						self.pac_left = false;
					}
				}
			}
			else if length == 10 {
				if x > x + width {
					if self.pac_loc[0].0 >= x + width && self.pac_loc[0].0 <= x
						&& self.pac_loc[0].1 + 25 >= y && self.pac_loc[0].1 < y && self.pac_down {
						self.pac_down = false;
					}
					else if self.pac_loc[0].0 >= x + width && self.pac_loc[0].0 <= x 
						&& self.pac_loc[0].1 - 25 <= y && self.pac_loc[0].1 > y && self.pac_up {
						self.pac_up = false;
					}
				}
				else if x < x + width {
					if self.pac_loc[0].0 >= x && self.pac_loc[0].0 <= x + width
						&& self.pac_loc[0].1 + 25 >= y && self.pac_loc[0].1 < y && self.pac_down {
						self.pac_down = false;
					}
					else if self.pac_loc[0].0 >= x && self.pac_loc[0].0 <= x + width
						&& self.pac_loc[0].1 - 25 <= y && self.pac_loc[0].1 > y && self.pac_up {
						self.pac_up = false;
					}
				}
			}
		} 

	}

	pub fn run(&mut self) {

		self.init_sprites();

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
					self.check_collision();
					self.check_pellet();

					if !self.check_ghost_collision() && self.lives > 0 {

						if self.pac_up {
							self.pac_loc[0].1 = self.pac_loc[0].1 - 1;
							self.pac_direction = 1;
						}
						else if self.pac_down {
							self.pac_loc[0].1 = self.pac_loc[0].1 + 1;
							self.pac_direction = 2;
						}
						else if self.pac_left {
							self.pac_loc[0].0 = self.pac_loc[0].0 - 1;
							self.pac_direction = 3;
						}
						else if self.pac_right {
							self.pac_loc[0].0 = self.pac_loc[0].0 + 1;
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
					else {
						self.update_lives();
					}
				},
				_ => {

				}
			}
			
		}
	}
}