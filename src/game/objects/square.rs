use crate::game::GameObject;
use crate::game::constants::{SCREEN_HEIGHT, SCREEN_WIDTH, VELOCITY_RANGE};
use rand::Rng;
use raylib::prelude::*;

pub struct Square {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    color: Color,
    xvel: i32,
    yvel: i32,
}

impl Square {
    pub fn init(width: i32, height: i32, color: Color) -> Square {
        let mut rng = rand::rng();
        Square {
            x: rng.random_range(0..SCREEN_WIDTH - width),
            y: rng.random_range(0..SCREEN_HEIGHT - height),
            width,
            height,
            color,
            xvel: rng.random_range(VELOCITY_RANGE),
            yvel: rng.random_range(VELOCITY_RANGE),
        }
    }
}

impl GameObject for Square {
    fn update(&mut self, d: &mut RaylibDrawHandle) {
        self.x += self.xvel;
        if self.x < 0 || (self.x + self.width) > d.get_screen_width() {
            self.xvel *= -1;
        }

        self.y += self.yvel;
        if self.y < 0 || (self.y + self.height) > d.get_screen_height() {
            self.yvel *= -1;
        }
    }

    fn draw(&mut self, d: &mut RaylibDrawHandle) {
        d.draw_rectangle(self.x, self.y, self.width, self.height, self.color);
    }
}
