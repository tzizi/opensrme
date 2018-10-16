use std::ops::*;

pub type Time = u64;

#[derive(Debug)]
pub struct Color {
  pub r: u8,
  pub g: u8,
  pub b: u8,
  pub a: u8
}

impl Color {
  pub fn to_rgba(&self) -> u32 {
    ((self.a as u32) << 24) |
    ((self.b as u32) << 16) |
    ((self.g as u32) << 8)  |
    (self.r as u32)
  }

  pub fn from_rgba(rgba: u32) -> Self {
    Color {
      r: ((rgba >> 0)  & 0xff) as u8,
      g: ((rgba >> 8)  & 0xff) as u8,
      b: ((rgba >> 16) & 0xff) as u8,
      a: ((rgba >> 24) & 0xff) as u8
    }
  }
}

#[derive(Debug)]
pub struct Image {
  pub data: Vec<u8>,
  pub size: Vec2i
}

#[derive(Debug, PartialEq)]
pub struct Vec2<T> {
  pub x: T,
  pub y: T
}

impl<T> Vec2<T> {
  pub fn new(x: T, y: T) -> Self {
    Vec2::<T> {
      x,
      y
    }
  }
}

pub type IScalar = i32;
pub type FScalar = f64;

pub type Vec2i = Vec2<IScalar>;
pub type Vec2f = Vec2<FScalar>;

impl<T: Add<Output=T>> Add<Vec2<T>> for Vec2<T> {
  type Output = Vec2<T>;

  fn add(self, other: Vec2<T>) -> Vec2<T> {
    Vec2::<T> {
      x: self.x + other.x,
      y: self.y + other.y
    }
  }
}

impl<T: Add<Output=T>+Copy> Add<T> for Vec2<T> {
  type Output = Vec2<T>;

  fn add(self, other: T) -> Vec2<T> {
    Vec2::<T> {
      x: self.x + other,
      y: self.y + other
    }
  }
}

impl<T: Sub<Output=T>> Sub for Vec2<T> {
  type Output = Vec2<T>;

  fn sub(self, other: Vec2<T>) -> Vec2<T> {
    Vec2::<T> {
      x: self.x - other.x,
      y: self.y - other.y
    }
  }
}

impl<T: Sub<Output=T>+Copy> Sub<T> for Vec2<T> {
  type Output = Vec2<T>;

  fn sub(self, other: T) -> Vec2<T> {
    Vec2::<T> {
      x: self.x - other,
      y: self.y - other
    }
  }
}

impl<T: Mul<Output=T>> Mul<Vec2<T>> for Vec2<T> {
  type Output = Vec2<T>;

  fn mul(self, other: Vec2<T>) -> Vec2<T> {
    Vec2::<T> {
      x: self.x * other.x,
      y: self.y * other.y
    }
  }
}

impl<T: Mul<Output=T>+Copy> Mul<T> for Vec2<T> {
  type Output = Vec2<T>;

  fn mul(self, other: T) -> Vec2<T> {
    Vec2::<T> {
      x: self.x * other,
      y: self.y * other
    }
  }
}

impl<T: Div<Output=T>> Div<Vec2<T>> for Vec2<T> {
  type Output = Vec2<T>;

  fn div(self, other: Vec2<T>) -> Vec2<T> {
    Vec2::<T> {
      x: self.x / other.x,
      y: self.y / other.y
    }
  }
}

impl<T: Div<Output=T>+Copy> Div<T> for Vec2<T> {
  type Output = Vec2<T>;

  fn div(self, other: T) -> Vec2<T> {
    Vec2::<T> {
      x: self.x / other,
      y: self.y / other
    }
  }
}
