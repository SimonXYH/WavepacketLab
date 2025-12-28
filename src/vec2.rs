use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Clone, Copy)]

pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Vec2 { x, y }
    }

    pub fn into_tuple(self) -> (T, T) {
        (self.x, self.y)
    }
}

impl Vec2<f32> {
    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn normalize(&mut self) -> Self {
        let length = self.length();
        Vec2::new(self.x / length, self.y / length)
    }

    pub fn dot(self, other: Self) -> f32 {
        (self.x * other.x + self.y * other.y)
    }

    pub fn distance_squared(self, other: Self) -> f32 {
        (self.x - other.x).powi(2) + (self.y - other.y).powi(2)
    }

    pub fn distance(self, other: Self) -> f32 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

impl<T: Add<T, Output = T>> Add for Vec2<T> {
    type Output = Vec2<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Vec2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T: Neg<Output = T>> Neg for Vec2<T> {
    type Output = Vec2<T>;
    fn neg(self) -> Self::Output {
        Vec2 {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl<T: Sub<T, Output = T>> Sub for Vec2<T> {
    type Output = Vec2<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<U: Copy, T: std::ops::Mul<U>> Mul<U> for Vec2<T> {
    type Output = Vec2<<T as Mul<U>>::Output>;

    fn mul(self, rhs: U) -> Self::Output {
        Vec2::new(self.x * rhs, self.y * rhs)
    }
}

impl<U: Copy, T: std::ops::Div<U>> Div<U> for Vec2<T> {
    type Output = Vec2<<T as Div<U>>::Output>;

    fn div(self, rhs: U) -> Self::Output {
        Vec2::new(self.x / rhs, self.y / rhs)
    }
}
