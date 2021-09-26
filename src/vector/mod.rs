mod test;

use std::ops::{Add, Sub};

struct Vector {
    x: f32,
    y: f32,
}

impl Vector {
    fn new(x: f32, y: f32) -> Vector {
        Vector { x, y }
    }

    fn origin() -> Vector {
        Vector { x: 0f32, y: 0f32 }
    }

    fn print(&self) {
        println!("Vector: {}, {}", self.x, self.y);
    }

    fn add_values(&mut self, x: f32, y: f32) {
        self.x += x;
        self.y += y;
    }

    fn sub_values(&mut self, x: f32, y: f32) {
        self.x -= x;
        self.y -= y;
    }

    fn add_vector(&mut self, other: &Vector) {
        self.x += other.x;
        self.y += other.y;
    }

    fn sub_vector(&mut self, other: &Vector) {
        self.x -= other.x;
        self.y -= other.y;
    }

    fn mag(&self) -> f32 {
        (self.x.powf(2f32) + self.y.powf(2f32)).sqrt()
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, other: Vector) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        return if self.x == other.x && self.y == other.y {
            true
        } else {
            false
        };
    }
}
