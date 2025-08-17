#![allow(unused)]

mod constants;
mod objects;

use constants::{SCREEN_HEIGHT, SCREEN_WIDTH};
use objects::{Circle, Square};
use raylib::prelude::*;

enum GameState {
    Idle,
    Running,
    Finished,
}

trait GameObject {
    fn update(&mut self, d: &mut RaylibDrawHandle);
    fn draw(&mut self, d: &mut RaylibDrawHandle);
}

pub fn run() {
    // Initialize raylib and window
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Game Template")
        .log_level(TraceLogLevel::LOG_ERROR)
        .vsync()
        .build();

    rl.set_target_fps(60);

    let state = GameState::Idle;

    // Register all game objects that will be updated and drawn
    let mut objects: Vec<Box<dyn GameObject>> = Vec::new();
    objects.push(Box::new(Square::init(150, 200, Color::RED)));
    objects.push(Box::new(Square::init(160, 150, Color::WHITESMOKE)));
    objects.push(Box::new(Square::init(300, 210, Color::GREENYELLOW)));
    objects.push(Box::new(Square::init(158, 221, Color::CYAN)));
    objects.push(Box::new(Circle::init(40, Color::YELLOW)));
    objects.push(Box::new(Circle::init(50, Color::BLACK)));
    objects.push(Box::new(Circle::init(80, Color::BLUE)));

    // Game loop
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::PINK);

        // Update all game objects
        for obj in &mut objects {
            obj.update(&mut d);
        }

        if let GameState::Finished = state {
            break;
        }

        // Draw all game objects
        for obj in &mut objects {
            obj.draw(&mut d);
        }
    }
}
