use crate::game::GameObject;
use crate::game::constants::{SCREEN_HEIGHT, SCREEN_WIDTH, VELOCITY_RANGE};
use rand::Rng;
use raylib::prelude::*;

pub struct Circle {
    x: i32,
    y: i32,
    radius: i32,
    color: Color,
    xvel: i32,
    yvel: i32,
}

impl Circle {
    pub fn init(radius: i32, color: Color) -> Circle {
        let mut rng = rand::rng();
        Circle {
            x: rng.random_range(radius..SCREEN_WIDTH - radius),
            y: rng.random_range(radius..SCREEN_HEIGHT - radius),
            radius,
            color,
            xvel: rng.random_range(VELOCITY_RANGE),
            yvel: rng.random_range(VELOCITY_RANGE),
        }
    }
}

impl GameObject for Circle {
    fn update(&mut self, d: &mut RaylibDrawHandle) {
        self.x += self.xvel;
        if self.x < self.radius || (self.x + self.radius) > d.get_screen_width() {
            self.xvel *= -1;
        }

        self.y += self.yvel;
        if self.y < self.radius || (self.y + self.radius) > d.get_screen_height() {
            self.yvel *= -1;
        }
    }

    fn draw(&mut self, d: &mut RaylibDrawHandle) {
        d.draw_circle(self.x, self.y, self.radius as f32, self.color);
    }
}
