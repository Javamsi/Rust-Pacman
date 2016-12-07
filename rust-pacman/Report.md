Rust Pac Man
===================
## Summary

Rust Pac Man is a Rust implementation of the classic arcade game Pac Man. The player controls Pac Man through a maze to consume pellets, which in turn give points. The goal is to consume all of the pellets without losing all of your lives, so you must avoid the ghosts chasing you. You can consume mega pellets to scare off the ghosts and cause them to change into frightened mode. Once in frightened mode, Pac Man can consume them, in which case they will return home. Once ghosts return home, they chase again. The game is over once all of the pellets have been consumed or Pac Man loses all of his lives. The game uses Piston, a modular open source game engine for Rust, to establish the board and all of the drawable components in the game as well as to render 2d images.


----------


Accomplishments
-------------

A game board designed using Piston Window and matching that of Pac Man including pellets, intersections, score, lives, and game objects.
The game parses the intersections, coordinates, pellets, and mega pellets files and stores them in Vecs to be rendered and updated in the game. 
Using intersection locations for Pac Man and ghost movement as well as decision making. Able to handle collisions between ghosts, pacman, and walls and prevents any object from exiting the game.
Follows the ghost protocol for chasing pacman, running away, scattering, and returning to their home/quadrants.
 Ghost states change based on interactions and time as wel as their images to have real time interactions with the reset of the game.

Problems Encountered
--------------------------------

Handling coordinates in Piston Window
Handling collision between characters and rectangles
Lag on movement when sprites/images were added
Using the draw_2d method properly.
Causes lag when draw is called within a loop or multiple draws are called. 
Avoided by borrowing drawable content and looping within draw
Consuming pellets increased originally increased movement speed, needed to reorganize drawable features.

Benefits of Rust
--------------------------------
Piston - Modular Open source game engine
Well documented Piston 2D and 3D graphics API
Nice examples of 2D graphics and movement
Simple Game code structure
Separation of input, render, and update 
Easy use of window, draw_2d, and state updates
High performance and Memory Safe

Components
--------------------------------

###Game 
--------------------------------------
Initialize Piston window, pac man, ghosts by  parsing file containing coordinates of walls, intersections, and pellets. 
Initialize all of the sprites necessary  Create the necessary functions required by the game to check collisions with walls, check collisions with ghosts, 
Create run loop through Piston Window and listen for events

>3 different types of events possible:
Input - Parse the input by the user (up, down, left, right)
Render - Draw state of the game
Update - Update the game fields and state

Input: Recieve keyboard input from user and update Pac Man's direction
Draw: Draw each object based on its current state.
Update: Update locations, score, directions, collision checks, and call chase function for ghosts. Check collision with walls and between characters and check for the end game state.

###Ghost
--------------------------------------
Maintain a reference to Pac Man’s location
Maintain references to all other game objects
Call Chase function at Game update
Check for state changes and update target tile
get distance functions 
Number of available directions
Find shortest distance to target tile
Update direction 

###Ghost AI
--------------------------------------
All ghosts start at their home and constantly move
Targeting Scheme - Target Location/Tile
Each ghost approaches its target tile based on its “personality”

Ghosts can alternate between 4 states:
Chase - Chase Pac Man
Scatter - Retreat to their quadrants
Frightened - Run in fear randomly
Home - Run home if caught
   -	Alternate between Chase and Scattered	
   -	Evaluate target tile at intersection

####Blinky
--------------------------------------
Targets Pac Man’s Location
Short-sighted vision can cause it to go in the wrong direction at intersections.
Implemented by setting the target tile to Pac Man's current location during chase.

####Pinky
--------------------------------------
Look at Pac Man’s current direction and location and target four tiles ahead.
Can be tricked when Pac Man looks a particular way or gets near as Pinky approaches an intersection

####Inky
--------------------------------------
Extends a vector between Blinky and Pac Man
Targeting can vary if Blinky is not near Pac Man, but ambushes/traps Pac Man when Blinky is near

####Clyde
--------------------------------------
If farther than a particular distance, target Pac Man
Else set to scatter mode target
Can be blocked into a loop by staying near
Avoid Clyde’s Quadrant if he’s near there


Improvements/Modifications
-----------------------------------------
Refactor code, factor out draw and initialize methods
Find more efficient way to handle creation and use of sprites
Add more animations
Changing ghosts to white before exiting frightened mode
Pac Man’s death animation
Improve ghost-pacman collision
Add bonus features (cherries, 1up, etc..)
Add more levels


