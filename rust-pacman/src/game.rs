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
	mega_pellets: Vec<(i32, i32)>,
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
	pacman_sprite_close: Option<Texture<Resources>>,
	pacman_sprite_up_full: Option<Texture<Resources>>,
	pacman_sprite_down_full: Option<Texture<Resources>>,
	pacman_sprite_left_full: Option<Texture<Resources>>,
	pacman_sprite_right_full: Option<Texture<Resources>>,
	pacman_sprite_up: Option<Texture<Resources>>,
	pacman_sprite_down: Option<Texture<Resources>>,
	pacman_sprite_left: Option<Texture<Resources>>,
	pacman_sprite_right: Option<Texture<Resources>>,
	blinky_sprite_up: Option<Texture<Resources>>,
	blinky_sprite_down: Option<Texture<Resources>>,
	blinky_sprite_left: Option<Texture<Resources>>,
	blinky_sprite_right: Option<Texture<Resources>>,
	pinky_sprite_up: Option<Texture<Resources>>,
	pinky_sprite_down: Option<Texture<Resources>>,
	pinky_sprite_left: Option<Texture<Resources>>,
	pinky_sprite_right: Option<Texture<Resources>>,
	inky_sprite_up: Option<Texture<Resources>>,
	inky_sprite_down: Option<Texture<Resources>>,
	inky_sprite_right: Option<Texture<Resources>>,
	inky_sprite_left: Option<Texture<Resources>>,
	clyde_sprite_up: Option<Texture<Resources>>,
	clyde_sprite_down: Option<Texture<Resources>>,
	clyde_sprite_left: Option<Texture<Resources>>,
	clyde_sprite_right: Option<Texture<Resources>>,
	ghost_feared_sprite: Option<Texture<Resources>>,
	ghost_home_up: Option<Texture<Resources>>,
	ghost_home_down: Option<Texture<Resources>>,
	ghost_home_left: Option<Texture<Resources>>,
	ghost_home_right: Option<Texture<Resources>>,
	pac_counter: i32,
	pac_name: String,
	game_score: i32,
	glyphs: Option<Glyphs>,
	normal_text: Option<Glyphs>,
	lives: i32,
	timer: i32,
	ghost_fear_timer: i32,
	feared: bool,
	game_won: bool,
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

		let mut mega_pellets = Vec::<(i32, i32)>::with_capacity(2);

		// Add Megapellets 
		mega_pellets.push((35,95));
		mega_pellets.push((875,95));
		mega_pellets.push((875,745));
		mega_pellets.push((35,745));

		/* Initialize ghost location */
		let mut blinky: Ghost = Ghost::new(String::from("blinky"), 365, 435);
		blinky.set_home(365, 435);
		let mut pinky: Ghost = Ghost::new(String::from("pinky"), 425, 435);
		pinky.set_home(425,435);
		let mut inky: Ghost = Ghost::new(String::from("inky"), 485, 435);
		inky.set_home(485, 435);
		let mut clyde: Ghost = Ghost::new(String::from("clyde"), 545, 435);
		clyde.set_home(545, 435);

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
			pacman_sprite_close: None,
			pacman_sprite_up_full: None,
			pacman_sprite_down_full: None,
			pacman_sprite_left_full: None,
			pacman_sprite_right_full: None,
			pacman_sprite_up: None,
			pacman_sprite_down: None,
			pacman_sprite_left: None,
			pacman_sprite_right: None,
			blinky_sprite_up: None,
			blinky_sprite_down: None,
			blinky_sprite_left: None,
			blinky_sprite_right: None,
			pinky_sprite_up: None,
			pinky_sprite_down: None,
			pinky_sprite_left: None,
			pinky_sprite_right: None,
			inky_sprite_up: None,
			inky_sprite_down: None,
			inky_sprite_right: None,
			inky_sprite_left: None,
			clyde_sprite_up: None,
			clyde_sprite_down: None,
			clyde_sprite_left: None,
			clyde_sprite_right: None,
			ghost_feared_sprite: None,
			ghost_home_up: None,
			ghost_home_down: None,
			ghost_home_left: None,
			ghost_home_right: None,
			pac_counter: 0,
			pac_name: String::from("pac_right.png"),
			game_score: 0,
			glyphs: None,
			normal_text: None,
			lives: 3,
			timer: 0,
			mega_pellets: mega_pellets,
			ghost_fear_timer: 0,
			feared: false,
			game_won: false,
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

		/* Pac Man Sprite Close */
		let pacman_sprite_close = assets.join("pac_close.png");
		let pacman_sprite_close = Texture::from_path(
			&mut self.window.factory,
			&pacman_sprite_close,
			Flip::None,
			&TextureSettings::new())
			.unwrap();
		self.pacman_sprite_close = Some(pacman_sprite_close);

		/* Pac Man Sprite Up Full */
		let pacman_sprite_up_full = assets.join("pac_up_full.png");
		let pacman_sprite_up_full = Texture::from_path(
			&mut self.window.factory,
			&pacman_sprite_up_full,
			Flip::None,
			&TextureSettings::new())
			.unwrap();
		self.pacman_sprite_up_full = Some(pacman_sprite_up_full);

		/* Pac Man Sprite Down Full */
		let pacman_sprite_down_full = assets.join("pac_down_full.png");
		let pacman_sprite_down_full  = Texture::from_path(
			&mut self.window.factory,
			&pacman_sprite_down_full,
			Flip::None,
			&TextureSettings::new())
			.unwrap();
		self.pacman_sprite_down_full = Some(pacman_sprite_down_full);

		/* Pac Man Sprite Left Full*/
		let pacman_sprite_left_full = assets.join("pac_left_full.png");
		let pacman_sprite_left_full = Texture::from_path(
			&mut self.window.factory,
			&pacman_sprite_left_full,
			Flip::None,
			&TextureSettings::new())
			.unwrap();
		self.pacman_sprite_left_full = Some(pacman_sprite_left_full);

		/* Pac Man Sprite Right Full */
		let pacman_sprite_right_full = assets.join("pac_right_full.png");
		let pacman_sprite_right_full = Texture::from_path(
			&mut self.window.factory,
			&pacman_sprite_right_full,
			Flip::None,
			&TextureSettings::new())
			.unwrap();
		self.pacman_sprite_right_full = Some(pacman_sprite_right_full);

		/* Pac Man Sprite Up Close*/
		let pacman_sprite_up = assets.join("pac_up.png");
		let pacman_sprite_up = Texture::from_path(
			&mut self.window.factory,
			&pacman_sprite_up,
			Flip::None,
			&TextureSettings::new())
			.unwrap();
		self.pacman_sprite_up = Some(pacman_sprite_up);

		/* Pac Man Sprite Down Close*/
		let pacman_sprite_down = assets.join("pac_down.png");
		let pacman_sprite_down  = Texture::from_path(
			&mut self.window.factory,
			&pacman_sprite_down,
			Flip::None,
			&TextureSettings::new())
			.unwrap();
		self.pacman_sprite_down = Some(pacman_sprite_down);

		/* Pac Man Sprite Left Close */
		let pacman_sprite_left = assets.join("pac_left.png");
		let pacman_sprite_left = Texture::from_path(
			&mut self.window.factory,
			&pacman_sprite_left,
			Flip::None,
			&TextureSettings::new())
			.unwrap();
		self.pacman_sprite_left = Some(pacman_sprite_left);

		/* Pac Man Sprite Right Close */
		let pacman_sprite_right = assets.join("pac_right.png");
		let pacman_sprite_right = Texture::from_path(
			&mut self.window.factory,
			&pacman_sprite_right,
			Flip::None,
			&TextureSettings::new())
			.unwrap();
		self.pacman_sprite_right = Some(pacman_sprite_right);

		/* Blinky Sprite Up */
		let blinky_sprite_up = assets.join("blinky_up.png");
		let blinky_sprite_up = Texture::from_path(
			&mut self.window.factory,
			&blinky_sprite_up,
			Flip::None,
			&TextureSettings::new())
			.unwrap();
		self.blinky_sprite_up = Some(blinky_sprite_up);

		/* Pac Man Sprite Down */
		let blinky_sprite_down = assets.join("blinky_down.png");
		let blinky_sprite_down  = Texture::from_path(
			&mut self.window.factory,
			&blinky_sprite_down,
			Flip::None,
			&TextureSettings::new())
			.unwrap();
		self.blinky_sprite_down = Some(blinky_sprite_down);

		/* Pac Man Sprite left */
		let blinky_sprite_left = assets.join("blinky_left.png");
		let blinky_sprite_left = Texture::from_path(
			&mut self.window.factory,
			&blinky_sprite_left,
			Flip::None,
			&TextureSettings::new())
			.unwrap();
		self.blinky_sprite_left = Some(blinky_sprite_left);

		/* Pac Man Sprite Right */
		let blinky_sprite_right = assets.join("blinky_right.png");
		let blinky_sprite_right = Texture::from_path(
			&mut self.window.factory,
			&blinky_sprite_right,
			Flip::None,
			&TextureSettings::new())
			.unwrap();
		self.blinky_sprite_right = Some(blinky_sprite_right);

		/* Pinky Sprite Up */
		let pinky_sprite_up = assets.join("pinky_up.png");
		let pinky_sprite_up = Texture::from_path(
			&mut self.window.factory,
			&pinky_sprite_up,
			Flip::None,
			&TextureSettings::new())
			.unwrap();
		self.pinky_sprite_up = Some(pinky_sprite_up);

		/* Pinky Sprite Down */
		let pinky_sprite_down = assets.join("pinky_down.png");
		let pinky_sprite_down  = Texture::from_path(
			&mut self.window.factory,
			&pinky_sprite_down,
			Flip::None,
			&TextureSettings::new())
			.unwrap();
		self.pinky_sprite_down = Some(pinky_sprite_down);

		/* Pinky Sprite left */
		let pinky_sprite_left = assets.join("pinky_left.png");
		let pinky_sprite_left = Texture::from_path(
			&mut self.window.factory,
			&pinky_sprite_left,
			Flip::None,
			&TextureSettings::new())
			.unwrap();
		self.pinky_sprite_left = Some(pinky_sprite_left);

		/* Pinky Sprite Right */
		let pinky_sprite_right = assets.join("pinky_right.png");
		let pinky_sprite_right = Texture::from_path(
			&mut self.window.factory,
			&pinky_sprite_right,
			Flip::None,
			&TextureSettings::new())
			.unwrap();
		self.pinky_sprite_right = Some(pinky_sprite_right);

		/* Inky Sprite Up */
		let inky_sprite_up = assets.join("inky_up.png");
		let inky_sprite_up = Texture::from_path(
			&mut self.window.factory,
			&inky_sprite_up,
			Flip::None,
			&TextureSettings::new())
			.unwrap();
		self.inky_sprite_up = Some(inky_sprite_up);

		/* Inky Sprite Down */
		let inky_sprite_down = assets.join("inky_down.png");
		let inky_sprite_down  = Texture::from_path(
			&mut self.window.factory,
			&inky_sprite_down,
			Flip::None,
			&TextureSettings::new())
			.unwrap();
		self.inky_sprite_down = Some(inky_sprite_down);

		/* Inky Sprite left */
		let inky_sprite_left = assets.join("inky_left.png");
		let inky_sprite_left = Texture::from_path(
			&mut self.window.factory,
			&inky_sprite_left,
			Flip::None,
			&TextureSettings::new())
			.unwrap();
		self.inky_sprite_left = Some(inky_sprite_left);

		/* Inky Sprite Right */
		let inky_sprite_right = assets.join("inky_right.png");
		let inky_sprite_right = Texture::from_path(
			&mut self.window.factory,
			&inky_sprite_right,
			Flip::None,
			&TextureSettings::new())
			.unwrap();
		self.inky_sprite_right = Some(inky_sprite_right);

		/* Clyde Sprite Up*/
		let clyde_sprite_up = assets.join("clyde_up.png");
		let clyde_sprite_up = Texture::from_path(
			&mut self.window.factory,
			&clyde_sprite_up,
			Flip::None,
			&TextureSettings::new())
			.unwrap(); 
		self.clyde_sprite_up = Some(clyde_sprite_up);

		/* Clyde Sprite Down*/
		let clyde_sprite_down = assets.join("clyde_down.png");
		let clyde_sprite_down = Texture::from_path(
			&mut self.window.factory,
			&clyde_sprite_down,
			Flip::None,
			&TextureSettings::new())
			.unwrap(); 
		self.clyde_sprite_down = Some(clyde_sprite_down);

		/* Clyde Sprite Left */
		let clyde_sprite_left = assets.join("clyde_left.png");
		let clyde_sprite_left = Texture::from_path(
			&mut self.window.factory,
			&clyde_sprite_left,
			Flip::None,
			&TextureSettings::new())
			.unwrap(); 
		self.clyde_sprite_left = Some(clyde_sprite_left);

		/* Clyde Sprite Up*/
		let clyde_sprite_right = assets.join("clyde_right.png");
		let clyde_sprite_right = Texture::from_path(
			&mut self.window.factory,
			&clyde_sprite_right,
			Flip::None,
			&TextureSettings::new())
			.unwrap(); 
		self.clyde_sprite_right = Some(clyde_sprite_right);

		/* Ghost Feared Sprite */
		let ghost_feared_sprite = assets.join("ghost_feared.png");
		let ghost_feared_sprite = Texture::from_path(
			&mut self.window.factory,
			&ghost_feared_sprite,
			Flip::None,
			&TextureSettings::new())
			.unwrap();
		self.ghost_feared_sprite = Some(ghost_feared_sprite);

		/* Ghost Home Up */
		let ghost_home_up = assets.join("home_up.png");
		let ghost_home_up = Texture::from_path(
			&mut self.window.factory,
			&ghost_home_up,
			Flip::None,
			&TextureSettings::new())
			.unwrap();
		self.ghost_home_up = Some(ghost_home_up);

		/* Ghost Home Down */
		let ghost_home_down = assets.join("home_down.png");
		let ghost_home_down = Texture::from_path(
			&mut self.window.factory,
			&ghost_home_down,
			Flip::None,
			&TextureSettings::new())
			.unwrap();
		self.ghost_home_down = Some(ghost_home_down);

		/* Ghost Home Left */
		let ghost_home_left = assets.join("home_left.png");
		let ghost_home_left = Texture::from_path(
			&mut self.window.factory,
			&ghost_home_left,
			Flip::None,
			&TextureSettings::new())
			.unwrap();
		self.ghost_home_left = Some(ghost_home_left);

		/* Ghost Home Right */
		let ghost_home_right = assets.join("home_right.png");
		let ghost_home_right = Texture::from_path(
			&mut self.window.factory,
			&ghost_home_right,
			Flip::None,
			&TextureSettings::new())
			.unwrap();
		self.ghost_home_right = Some(ghost_home_right);

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
		let mut mega_pellets: Vec<(i32, i32)> = Vec::new();
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

		/* Pac Man Location*/
		for &(x,y) in &self.pac_loc { pac_loc.push((x,y)); }

		/* Pellets */
		for &(x,y) in &self.pellets { pellets.push((x,y)); }

		/* Mega Pellets */
		for &(x,y) in &self.mega_pellets { mega_pellets.push((x,y)); }

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

        /* Draw the Mega Pellets */
        self.window.draw_2d(&e, |c, g| {
           	for (x,y) in mega_pellets {
          		rectangle([1.0, 1.0, 0.0, 1.0], 
                     [(x - 3) as f64, (y - 3) as f64, 20.0, 20.0],
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
        		if (self.game_won) {
        			self.window.draw_2d(&e, |c, g| {
					let transform = c.transform.trans(370.0, 230.0);
		   			text::Text::new_color([1.0, 1.0, 0.0, 1.0], 16).draw(
            		&"YOU WON!",
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

        /* Draw Lives */
        match self.pacman_sprite_right {
           	None => {}
           	Some(ref sprite) => {
           		/* Draw lives */
           		for i in 0..lives {
        			self.window.draw_2d(&e, |c, g| {
           				let center = c.transform.trans(40.0 + ((i as f64) * 30.0), 370.0 as f64);
           					image(sprite, center, g);
           			});
        		}
           	}
        }

        /* Draw Pac Man */

        if (self.pac_direction == 1) {
        	if (self.timer >= 0 && self.timer < 2) {
       			match self.pacman_sprite_up_full {
           			None => {}
           			Some(ref sprite) => {
           				self.window.draw_2d(&e, |c, g| {
           					let center = c.transform.trans((pac_loc[0].0 - 5)as f64, (pac_loc[0].1 - 5) as f64);
           					image(sprite, center, g);
           				});
           			}
        		}
        	}
        	else if (self.timer >= 2 && self.timer < 4) {
        		match self.pacman_sprite_up {
           			None => {}
           			Some(ref sprite) => {
           				self.window.draw_2d(&e, |c, g| {
           					let center = c.transform.trans((pac_loc[0].0 - 5)as f64, (pac_loc[0].1 - 5) as f64);
           					image(sprite, center, g);
           				});
           			}
        		}
        	}
        	else if (self.timer >= 4 && self.timer < 6) {
        		match self.pacman_sprite_close {
           			None => {}
           			Some(ref sprite) => {
           				self.window.draw_2d(&e, |c, g| {
           					let center = c.transform.trans((pac_loc[0].0 - 5)as f64, (pac_loc[0].1 - 5) as f64);
           					image(sprite, center, g);
           				});
           			}
        		}
        	}
        }	

       	else if (self.pac_direction == 2) {
        	if (self.timer >= 0  && self.timer < 2) {
       			match self.pacman_sprite_down_full {
           			None => {}
           			Some(ref sprite) => {
           				self.window.draw_2d(&e, |c, g| {
           					let center = c.transform.trans((pac_loc[0].0 - 5)as f64, (pac_loc[0].1 - 5) as f64);
           					image(sprite, center, g);
           				});
           			}
        		}
        	}
        	else if (self.timer >= 2 && self.timer < 4) {
        		match self.pacman_sprite_down {
           			None => {}
           			Some(ref sprite) => {
           				self.window.draw_2d(&e, |c, g| {
           					let center = c.transform.trans((pac_loc[0].0 - 5)as f64, (pac_loc[0].1 - 5) as f64);
           					image(sprite, center, g);
           				});
           			}
        		}
        	}
        	else if (self.timer >= 4 && self.timer < 6) {
        		match self.pacman_sprite_close {
           			None => {}
           			Some(ref sprite) => {
           				self.window.draw_2d(&e, |c, g| {
           					let center = c.transform.trans((pac_loc[0].0 - 5)as f64, (pac_loc[0].1 - 5) as f64);
           					image(sprite, center, g);
           				});
           			}
        		}
        	}
        }

        else if (self.pac_direction == 3) {
        	if (self.timer >= 0 && self.timer < 2) {
       			match self.pacman_sprite_left_full {
           			None => {}
           			Some(ref sprite) => {
           				self.window.draw_2d(&e, |c, g| {
           					let center = c.transform.trans((pac_loc[0].0 - 5)as f64, (pac_loc[0].1 - 5) as f64);
           					image(sprite, center, g);
           				});
           			}
        		}
        	}
        	else if (self.timer >= 2 && self.timer < 4) {
        		match self.pacman_sprite_left {
           			None => {}
           			Some(ref sprite) => {
           				self.window.draw_2d(&e, |c, g| {
           					let center = c.transform.trans((pac_loc[0].0 - 5)as f64, (pac_loc[0].1 - 5) as f64);
           					image(sprite, center, g);
           				});
           			}
        		}
        	}
        	else if (self.timer >= 4 && self.timer < 6) {
        		match self.pacman_sprite_close {
           			None => {}
           			Some(ref sprite) => {
           				self.window.draw_2d(&e, |c, g| {
           					let center = c.transform.trans((pac_loc[0].0 - 5)as f64, (pac_loc[0].1 - 5) as f64);
           					image(sprite, center, g);
           				});
           			}
        		}
        	}
        }

        else if (self.pac_direction == 4) {
        	if (self.timer >= 0 && self.timer < 2) {
       			match self.pacman_sprite_right_full {
           			None => {}
           			Some(ref sprite) => {
           				self.window.draw_2d(&e, |c, g| {
           					let center = c.transform.trans((pac_loc[0].0 - 5)as f64, (pac_loc[0].1 - 5) as f64);
           					image(sprite, center, g);
           				});
           			}
        		}
        	}
        	else if (self.timer >= 2 && self.timer < 4) {
        		match self.pacman_sprite_right {
           			None => {}
           			Some(ref sprite) => {
           				self.window.draw_2d(&e, |c, g| {
           					let center = c.transform.trans((pac_loc[0].0 - 5)as f64, (pac_loc[0].1 - 5) as f64);
           					image(sprite, center, g);
           				});
           			}
        		}
        	}
        	else if (self.timer >= 4 && self.timer < 6) {
        		match self.pacman_sprite_close {
           			None => {}
           			Some(ref sprite) => {
           				self.window.draw_2d(&e, |c, g| {
           					let center = c.transform.trans((pac_loc[0].0 - 5)as f64, (pac_loc[0].1 - 5) as f64);
           					image(sprite, center, g);
           				});
           			}
        		}
        	}
        }        
        self.timer = (self.timer + 1) % 6;

        /* Draw Blinky */
        if (self.blinky.get_state() == 1) {
       		match self.ghost_feared_sprite {
           		None => {}
           		Some(ref sprite) => {
           			self.window.draw_2d(&e, |c, g| {
           			let center = c.transform.trans((blinkyx - 5)as f64, (blinkyy - 5) as f64);
           				image(sprite, center, g);
           			});
           		}
        	}        	
        }
        else if (self.blinky.get_direction() == 1) {
        	if (self.blinky.get_state() == 3) {
       			match self.ghost_home_up {
           			None => {}
           			Some(ref sprite) => {
           				self.window.draw_2d(&e, |c, g| {
           				let center = c.transform.trans((blinkyx - 5)as f64, (blinkyy - 5) as f64);
           					image(sprite, center, g);
           				});
           			}
        		}        		
        	}
        	else {
       			match self.blinky_sprite_up {
           			None => {}
           			Some(ref sprite) => {
           				self.window.draw_2d(&e, |c, g| {
           				let center = c.transform.trans((blinkyx - 5)as f64, (blinkyy - 5) as f64);
           					image(sprite, center, g);
           				});
           			}
        		}
        	}
        }
        else if (self.blinky.get_direction() == 2) {
       		if (self.blinky.get_state() == 3) {
       			match self.ghost_home_down {
           			None => {}
           			Some(ref sprite) => {
           				self.window.draw_2d(&e, |c, g| {
           				let center = c.transform.trans((blinkyx - 5)as f64, (blinkyy - 5) as f64);
           					image(sprite, center, g);
           				});
           			}
        		}        		
        	}
        	else {
       			match self.blinky_sprite_down {
           			None => {}
           			Some(ref sprite) => {
           				self.window.draw_2d(&e, |c, g| {
           				let center = c.transform.trans((blinkyx - 5)as f64, (blinkyy - 5) as f64);
           					image(sprite, center, g);
           				});
           			}
        		}
        	}        	
        }
        else if (self.blinky.get_direction() == 3) {
       		if (self.blinky.get_state() == 3) {
       			match self.ghost_home_left {
           			None => {}
           			Some(ref sprite) => {
           				self.window.draw_2d(&e, |c, g| {
           				let center = c.transform.trans((blinkyx - 5)as f64, (blinkyy - 5) as f64);
           					image(sprite, center, g);
           				});
           			}
        		}        		
        	}
        	else {
       			match self.blinky_sprite_left {
           			None => {}
           			Some(ref sprite) => {
           				self.window.draw_2d(&e, |c, g| {
           				let center = c.transform.trans((blinkyx - 5)as f64, (blinkyy - 5) as f64);
           					image(sprite, center, g);
           				});
           			}
        		}
        	}       	
        }
        else if (self.blinky.get_direction() == 4) {
       		if (self.blinky.get_state() == 3) {
       			match self.ghost_home_right {
           			None => {}
           			Some(ref sprite) => {
           				self.window.draw_2d(&e, |c, g| {
           				let center = c.transform.trans((blinkyx - 5)as f64, (blinkyy - 5) as f64);
           					image(sprite, center, g);
           				});
           			}
        		}        		
        	}
        	else {
       			match self.blinky_sprite_right {
           			None => {}
           			Some(ref sprite) => {
           				self.window.draw_2d(&e, |c, g| {
           				let center = c.transform.trans((blinkyx - 5)as f64, (blinkyy - 5) as f64);
           					image(sprite, center, g);
           				});
           			}
        		}
        	}        	
        }

        /* Draw Pinky */
        if (self.pinky.get_state() == 1) {
       		match self.ghost_feared_sprite {
           		None => {}
           		Some(ref sprite) => {
           			self.window.draw_2d(&e, |c, g| {
           			let center = c.transform.trans((pinkyx - 5)as f64, (pinkyy - 5) as f64);
           				image(sprite, center, g);
           			});
           		}
        	}        	
        }
        else if (self.pinky.get_direction() == 1) {
       		if (self.pinky.get_state() == 3) {
       			match self.ghost_home_up {
           			None => {}
           			Some(ref sprite) => {
           				self.window.draw_2d(&e, |c, g| {
           				let center = c.transform.trans((pinkyx - 5)as f64, (pinkyy - 5) as f64);
           					image(sprite, center, g);
           				});
           			}
        		}        		
        	}
        	else {
       			match self.pinky_sprite_up {
           			None => {}
           			Some(ref sprite) => {
           				self.window.draw_2d(&e, |c, g| {
           				let center = c.transform.trans((pinkyx - 5)as f64, (pinkyy - 5) as f64);
           					image(sprite, center, g);
           				});
           			}
        		}
        	} 
        }
        else if (self.pinky.get_direction() == 2) {
       		if (self.pinky.get_state() == 3) {
       			match self.ghost_home_down {
           			None => {}
           			Some(ref sprite) => {
           				self.window.draw_2d(&e, |c, g| {
           				let center = c.transform.trans((pinkyx - 5)as f64, (pinkyy - 5) as f64);
           					image(sprite, center, g);
           				});
           			}
        		}        		
        	}
        	else {
       			match self.pinky_sprite_down {
           			None => {}
           			Some(ref sprite) => {
           				self.window.draw_2d(&e, |c, g| {
           				let center = c.transform.trans((pinkyx - 5)as f64, (pinkyy - 5) as f64);
           					image(sprite, center, g);
           				});
           			}
        		}
        	}         	
        }
        else if (self.pinky.get_direction() == 3) {
       		if (self.pinky.get_state() == 3) {
       			match self.ghost_home_left {
           			None => {}
           			Some(ref sprite) => {
           				self.window.draw_2d(&e, |c, g| {
           				let center = c.transform.trans((pinkyx - 5)as f64, (pinkyy - 5) as f64);
           					image(sprite, center, g);
           				});
           			}
        		}        		
        	}
        	else {
       			match self.pinky_sprite_left {
           			None => {}
           			Some(ref sprite) => {
           				self.window.draw_2d(&e, |c, g| {
           				let center = c.transform.trans((pinkyx - 5)as f64, (pinkyy - 5) as f64);
           					image(sprite, center, g);
           				});
           			}
        		}
        	}         	
        }
        else if (self.pinky.get_direction() == 4) {
       		if (self.pinky.get_state() == 3) {
       			match self.ghost_home_right {
           			None => {}
           			Some(ref sprite) => {
           				self.window.draw_2d(&e, |c, g| {
           				let center = c.transform.trans((pinkyx - 5)as f64, (pinkyy - 5) as f64);
           					image(sprite, center, g);
           				});
           			}
        		}        		
        	}
        	else {
       			match self.pinky_sprite_right {
           			None => {}
           			Some(ref sprite) => {
           				self.window.draw_2d(&e, |c, g| {
           				let center = c.transform.trans((pinkyx - 5)as f64, (pinkyy - 5) as f64);
           					image(sprite, center, g);
           				});
           			}
        		}
        	}         	
        }

        /* Draw Inky */
        if (self.inky.get_state() == 1) {
        	match self.ghost_feared_sprite {
           		None => {}
           		Some(ref sprite) => {
           			self.window.draw_2d(&e, |c, g| {
           			let center = c.transform.trans((inkyx - 5)as f64, (inkyy - 5) as f64);
           				image(sprite, center, g);
           			});
           		}
        	}       	
        }
        else if (self.inky.get_direction() == 1) {
       		if (self.inky.get_state() == 3) {
       			match self.ghost_home_down {
           			None => {}
           			Some(ref sprite) => {
           				self.window.draw_2d(&e, |c, g| {
           				let center = c.transform.trans((inkyx - 5)as f64, (inkyy - 5) as f64);
           					image(sprite, center, g);
           				});
           			}
        		}        		
        	}
        	else {
       			match self.inky_sprite_down {
           			None => {}
           			Some(ref sprite) => {
           				self.window.draw_2d(&e, |c, g| {
           				let center = c.transform.trans((inkyx - 5)as f64, (inkyy - 5) as f64);
           					image(sprite, center, g);
           				});
           			}
        		}
        	}
        }
        else if (self.inky.get_direction() == 2) {
       		if (self.inky.get_state() == 3) {
       			match self.ghost_home_down {
           			None => {}
           			Some(ref sprite) => {
           				self.window.draw_2d(&e, |c, g| {
           				let center = c.transform.trans((inkyx - 5)as f64, (inkyy - 5) as f64);
           					image(sprite, center, g);
           				});
           			}
        		}        		
        	}
        	else {
       			match self.inky_sprite_down {
           			None => {}
           			Some(ref sprite) => {
           				self.window.draw_2d(&e, |c, g| {
           				let center = c.transform.trans((inkyx - 5)as f64, (inkyy - 5) as f64);
           					image(sprite, center, g);
           				});
           			}
        		}
        	}        	
        }
        else if (self.inky.get_direction() == 3) {
       		if (self.inky.get_state() == 3) {
       			match self.ghost_home_left {
           			None => {}
           			Some(ref sprite) => {
           				self.window.draw_2d(&e, |c, g| {
           				let center = c.transform.trans((inkyx - 5)as f64, (inkyy - 5) as f64);
           					image(sprite, center, g);
           				});
           			}
        		}        		
        	}
        	else {
       			match self.inky_sprite_left {
           			None => {}
           			Some(ref sprite) => {
           				self.window.draw_2d(&e, |c, g| {
           				let center = c.transform.trans((inkyx - 5)as f64, (inkyy - 5) as f64);
           					image(sprite, center, g);
           				});
           			}
        		}
        	}       	
        }
        else if (self.inky.get_direction() == 4) {
       		if (self.inky.get_state() == 3) {       			
       			match self.ghost_home_right {
           			None => {}
           			Some(ref sprite) => {
           				self.window.draw_2d(&e, |c, g| {
           				let center = c.transform.trans((inkyx - 5)as f64, (inkyy - 5) as f64);
           					image(sprite, center, g);
           				});
           			}
        		}        		
        	}
        	else {
       			match self.inky_sprite_right {
           			None => {}
           			Some(ref sprite) => {
           				self.window.draw_2d(&e, |c, g| {
           				let center = c.transform.trans((inkyx - 5)as f64, (inkyy - 5) as f64);
           					image(sprite, center, g);
           				});
           			}
        		}
        	}        	
        }

       /* Draw Clyde */
        if (self.clyde.get_state() == 1) {
       		match self.ghost_feared_sprite {
           		None => {}
           		Some(ref sprite) => {
           			self.window.draw_2d(&e, |c, g| {
           			let center = c.transform.trans((clydex - 5)as f64, (clydey - 5) as f64);
           				image(sprite, center, g);
           			});
           		}
        	}        	
        }
        else if (self.clyde.get_direction() == 1) {
       		if (self.clyde.get_state() == 3) {
       			match self.ghost_home_up {
           			None => {}
           			Some(ref sprite) => {
           				self.window.draw_2d(&e, |c, g| {
           				let center = c.transform.trans((clydex - 5)as f64, (clydey - 5) as f64);
           					image(sprite, center, g);
           				});
           			}
        		}        		
        	}
        	else {
       			match self.clyde_sprite_up {
           			None => {}
           			Some(ref sprite) => {
           				self.window.draw_2d(&e, |c, g| {
           				let center = c.transform.trans((clydex - 5)as f64, (clydey - 5) as f64);
           					image(sprite, center, g);
           				});
           			}
        		}
        	}
        }
        else if (self.clyde.get_direction() == 2) {
       		if (self.clyde.get_state() == 3) {
       			match self.ghost_home_down {
           			None => {}
           			Some(ref sprite) => {
           				self.window.draw_2d(&e, |c, g| {
           				let center = c.transform.trans((clydex - 5)as f64, (clydey - 5) as f64);
           					image(sprite, center, g);
           				});
           			}
        		}        		
        	}
        	else {
       			match self.clyde_sprite_down {
           			None => {}
           			Some(ref sprite) => {
           				self.window.draw_2d(&e, |c, g| {
           				let center = c.transform.trans((clydex - 5)as f64, (clydey - 5) as f64);
           					image(sprite, center, g);
           				});
           			}
        		}
        	}       	
        }
        else if (self.clyde.get_direction() == 3) {
       		if (self.clyde.get_state() == 3) {
       			match self.ghost_home_left {
           			None => {}
           			Some(ref sprite) => {
           				self.window.draw_2d(&e, |c, g| {
           				let center = c.transform.trans((clydex - 5)as f64, (clydey - 5) as f64);
           					image(sprite, center, g);
           				});
           			}
        		}        		
        	}
        	else {
       			match self.clyde_sprite_left {
           			None => {}
           			Some(ref sprite) => {
           				self.window.draw_2d(&e, |c, g| {
           				let center = c.transform.trans((clydex - 5)as f64, (clydey - 5) as f64);
           					image(sprite, center, g);
           				});
           			}
        		}
        	}       	
        }
        else if (self.clyde.get_direction() == 4) {
       		if (self.clyde.get_state() == 3) {
       			match self.ghost_home_right {
           			None => {}
           			Some(ref sprite) => {
           				self.window.draw_2d(&e, |c, g| {
           				let center = c.transform.trans((clydex - 5)as f64, (clydey - 5) as f64);
           					image(sprite, center, g);
           				});
           			}
        		}        		
        	}
        	else {
       			match self.clyde_sprite_right {
           			None => {}
           			Some(ref sprite) => {
           				self.window.draw_2d(&e, |c, g| {
           				let center = c.transform.trans((clydex - 5)as f64, (clydey - 5) as f64);
           					image(sprite, center, g);
           				});
           			}
        		}
        	}        	
        }     	
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

		self.blinky.set_state(2);
		self.pinky.set_state(2);
		self.inky.set_state(2);
		self.clyde.set_state(2);
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
		counter = 0;
		for &(x,y) in &self.mega_pellets {
			if self.pac_loc[0].0 == x && self.pac_loc[0].1 == y {
				break;
			}
			counter = counter + 1;
		}

		if counter < self.mega_pellets.len() {
			self.blinky.set_state(1);
			self.pinky.set_state(1);
			self.inky.set_state(1);
			self.clyde.set_state(1);
			self.game_score = self.game_score + 200;
			self.mega_pellets.remove(counter); 
			self.feared = true;
		}
	}

	pub fn check_ghost_collision(&mut self) -> bool {

		let mut ghost_locations: Vec<(i32, i32, i32)> = Vec::<(i32, i32, i32)>::with_capacity(3);
		ghost_locations.push((self.blinky.get_loc().0, self.blinky.get_loc().1, 0));
		ghost_locations.push((self.pinky.get_loc().0, self.pinky.get_loc().1, 1));
		ghost_locations.push((self.inky.get_loc().0, self.inky.get_loc().1, 2));
		ghost_locations.push((self.clyde.get_loc().0, self.clyde.get_loc().1, 3));

		for &(x,y, ghost) in &ghost_locations {
			if self.pac_loc[0].1 == y {
				for counter in 0..18 {
					if self.pac_loc[0].0 + counter == x || self.pac_loc[0].0 - counter == x {
						if ghost == 0 {
							if self.blinky.get_state() == 2 { return true; }
							else if self.blinky.get_state() == 1 { 
								self.game_score = self.game_score + 400;
								self.blinky.set_state(3);
								return false;
							}
							else { return false; }
						}
						if ghost == 1 {
							if self.pinky.get_state() == 2 { return true; }
							else if self.pinky.get_state() == 1 { 
								self.game_score = self.game_score + 400;
								self.pinky.set_state(3);
								return false;
							}
							else { return false; }
						}
						if ghost == 2 {
							if self.inky.get_state() == 2 { return true; }
							else if self.inky.get_state() == 1 { 
								self.game_score = self.game_score + 400;
								self.inky.set_state(3);
								return false;
							}
							else { return false; }
						}
						if ghost == 3 {
							if self.clyde.get_state() == 2 { return true; }
							else if self.clyde.get_state() == 1 { 
								self.game_score = self.game_score + 400;
								self.clyde.set_state(3);
								return false;
							}
							else { return false; }
						}
					}
				}
			}
			else if self.pac_loc[0].0 == x {
				for counter in 0..18 {
					if self.pac_loc[0].1 + counter == y || self.pac_loc[0].1 - counter == y {
						if ghost == 0 {
							if self.blinky.get_state() == 2 { return true; }
							else if self.blinky.get_state() == 1 { 
								self.game_score = self.game_score + 400;
								self.blinky.set_state(3);
								return false;
							}
							else { return false; }
						}
						if ghost == 1 {
							if self.pinky.get_state() == 2 { return true; }
							else if self.pinky.get_state() == 1 { 
								self.game_score = self.game_score + 400;
								self.pinky.set_state(3);
								return false;
							}
							else { return false; }
						}
						if ghost == 2 {
							if self.inky.get_state() == 2 { return true; }
							else if self.inky.get_state() == 1 { 
								self.game_score = self.game_score + 400;
								self.inky.set_state(3);
								return false;
							}
							else { return false; }
						}
						if ghost == 3 {
							if self.clyde.get_state() == 2 { return true; }
							else if self.clyde.get_state() == 1 { 
								self.game_score = self.game_score + 400;
								self.clyde.set_state(3);
								return false;
							}
							else { return false; }
						}
					}
				}
			}
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

	pub fn check_game_won(&mut self) -> bool {
		let mut counter1: i32 = 0;
		let mut counter2: i32 = 0;
		for &(x,y) in &self.pellets {
			counter1 = counter1 + 1;
		}
		for &(x,y) in &self.mega_pellets {
			counter2 = counter2 + 1;
		}
		if counter2 + counter1 == 0 {
			self.game_won = true;
			return true;
		}
		return false;
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
					if self.feared {
						self.ghost_fear_timer = self.ghost_fear_timer + 1;
						if (self.ghost_fear_timer == 1000) {
							self.ghost_fear_timer = 0;
							self.feared = false;
							if self.blinky.get_state()== 1 {
								self.blinky.set_state(2);
							}
							if self.pinky.get_state() == 1 {
								self.pinky.set_state(2);
							}
							if self.inky.get_state() == 1 {
								self.inky.set_state(2);
							}
							if self.clyde.get_state() == 1 {
								self.clyde.set_state(2);
							} 
						}
					}
					self.check_collision();
					self.check_pellet();
					self.check_game_won();
					if !self.check_ghost_collision() && self.lives > 0 && !self.game_won {

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
						if self.pac_loc[0].0 == 875 && self.pac_loc[0].1 == 435 {
							self.pac_loc[0].0 = 35;
							self.pac_loc[0].1 = 435;
						}
						if self.pac_loc[0].0 == 35 && self.pac_loc[0].1 == 435 {
							self.pac_loc[0].0 = 875;
							self.pac_loc[0].1 = 435;
						}
					}
					else if self.game_won{

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