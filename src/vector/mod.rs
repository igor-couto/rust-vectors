mod test;

use std::ops::{Add, Sub};

pub struct Vector {
    x: f32,
    y: f32,
}

impl Vector {
    pub fn new(x: f32, y: f32) -> Vector {
        Vector { x, y }
    }

    pub fn origin() -> Vector {
        Vector { x: 0f32, y: 0f32 }
    }

    pub fn print(&self) {
        println!("Vector: {}, {}", self.x, self.y);
    }

    pub fn add_values(&mut self, x: f32, y: f32) {
        self.x += x;
        self.y += y;
    }

    pub fn sub_values(&mut self, x: f32, y: f32) {
        self.x -= x;
        self.y -= y;
    }

    pub fn add_vector(&mut self, other: &Vector) {
        self.x += other.x;
        self.y += other.y;
    }

    pub fn sub_vector(&mut self, other: &Vector) {
        self.x -= other.x;
        self.y -= other.y;
    }

    pub fn mag(&self) -> f32 {
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
