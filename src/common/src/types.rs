use std::ops::*;

pub type Time = u64;

#[derive(Debug, PartialEq, Copy, Clone)]
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

  pub fn from_bgr(bgr: u32) -> Self {
    Color {
      b: ((bgr >> 0)  & 0xff) as u8,
      g: ((bgr >> 8)  & 0xff) as u8,
      r: ((bgr >> 16) & 0xff) as u8,
      a: 255
    }
  }
}

#[derive(Debug)]
pub struct Image {
  pub data: Vec<u8>,
  pub size: Vec3i
}

pub type Angle = f32;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Vec3<T> {
  pub x: T,
  pub y: T,
  pub z: T
}

impl<T: Default+Into<f64>+Copy> Vec3<T> {
  pub fn new2(x: T, y: T) -> Self {
    Vec3::<T> {
      x,
      y,
      z: T::default()
    }
  }

  pub fn new3(x: T, y: T, z: T) -> Self {
    Vec3::<T> {
      x,
      y,
      z
    }
  }

  pub fn len2(&self) -> f64 {
    (self.x.into() * self.x.into() + self.y.into() * self.y.into()).sqrt()
  }

  pub fn len3(&self) -> f64 {
    (self.x.into() * self.x.into() + self.y.into() * self.y.into() + self.z.into() * self.z.into()).sqrt()
  }
}

pub type IScalar = i32;
pub type FScalar = f64;

pub type Vec3i = Vec3<IScalar>;
pub type Vec3f = Vec3<FScalar>;

impl From<Vec3i> for Vec3f {
  fn from(other: Vec3i) -> Self {
    Vec3f {
      x: other.x.into(),
      y: other.y.into(),
      z: other.z.into()
    }
  }
}

impl From<Vec3f> for Vec3i {
  fn from(other: Vec3f) -> Self {
    Vec3i {
      x: other.x as IScalar,
      y: other.y as IScalar,
      z: other.z as IScalar
    }
  }
}

impl<T: Add<Output=T>> Add<Vec3<T>> for Vec3<T> {
  type Output = Vec3<T>;

  fn add(self, other: Vec3<T>) -> Vec3<T> {
    Vec3::<T> {
      x: self.x + other.x,
      y: self.y + other.y,
      z: self.z + other.z
    }
  }
}

impl<T: Add<Output=T>+Copy> Add<T> for Vec3<T> {
  type Output = Vec3<T>;

  fn add(self, other: T) -> Vec3<T> {
    Vec3::<T> {
      x: self.x + other,
      y: self.y + other,
      z: self.z + other
    }
  }
}

impl<T: Sub<Output=T>> Sub for Vec3<T> {
  type Output = Vec3<T>;

  fn sub(self, other: Vec3<T>) -> Vec3<T> {
    Vec3::<T> {
      x: self.x - other.x,
      y: self.y - other.y,
      z: self.z - other.z
    }
  }
}

impl<T: Sub<Output=T>+Copy> Sub<T> for Vec3<T> {
  type Output = Vec3<T>;

  fn sub(self, other: T) -> Vec3<T> {
    Vec3::<T> {
      x: self.x - other,
      y: self.y - other,
      z: self.z - other
    }
  }
}

impl<T: Mul<Output=T>> Mul<Vec3<T>> for Vec3<T> {
  type Output = Vec3<T>;

  fn mul(self, other: Vec3<T>) -> Vec3<T> {
    Vec3::<T> {
      x: self.x * other.x,
      y: self.y * other.y,
      z: self.z * other.z
    }
  }
}

impl<T: Mul<Output=T>+Copy> Mul<T> for Vec3<T> {
  type Output = Vec3<T>;

  fn mul(self, other: T) -> Vec3<T> {
    Vec3::<T> {
      x: self.x * other,
      y: self.y * other,
      z: self.z * other
    }
  }
}

impl<T: Div<Output=T>> Div<Vec3<T>> for Vec3<T> {
  type Output = Vec3<T>;

  fn div(self, other: Vec3<T>) -> Vec3<T> {
    Vec3::<T> {
      x: self.x / other.x,
      y: self.y / other.y,
      z: self.z / other.z
    }
  }
}

impl<T: Div<Output=T>+Copy> Div<T> for Vec3<T> {
  type Output = Vec3<T>;

  fn div(self, other: T) -> Vec3<T> {
    Vec3::<T> {
      x: self.x / other,
      y: self.y / other,
      z: self.z / other
    }
  }
}
