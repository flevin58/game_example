### Description ###
This is an example on how to structure a game in rust using raylib.
It displays moving squares and circles.

Main features used here are:

- The main module simply calls game::run() because all the logic is implemented in mod game.
- A GameObject trait is used to gather all elements that can be updated and printed.
- A Vec<Box<dyn GameObject>> is used to push all objects, that are updated and displayes in respective loops.
- Simple to add other objects by creating a new file in src/game/objects/ with a struct that implements the GameObject trait.

### Screenshot ###

<img width="800" height="628" alt="Capto_Capture 2025-08-18_12-18-09_AM" src="https://github.com/user-attachments/assets/b450ff7a-ed0d-4577-9f92-12f2ccc637dc" />
