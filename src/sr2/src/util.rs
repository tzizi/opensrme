use rand::Rng;
use opensrme_common::*;
use super::*;
use std::ops::*;

pub const PI: Angle = std::f64::consts::PI;
pub const QUARTER_PI: Angle = PI / 4.;
pub const HALF_PI: Angle = PI / 2.;
pub const TWO_PI: Angle = PI * 2.;
pub const NAN: f64 = std::f64::NAN;

pub const TILESIZE: i32 = 24;

pub mod angles {
  use super::*;

  pub const ANGLE_E: Angle = 0.;
  pub const ANGLE_SE: Angle = QUARTER_PI;
  pub const ANGLE_S: Angle = HALF_PI;
  pub const ANGLE_SW: Angle = HALF_PI + QUARTER_PI;
  pub const ANGLE_W: Angle = PI;
  pub const ANGLE_NW: Angle = PI + QUARTER_PI;
  pub const ANGLE_N: Angle = PI + HALF_PI;
  pub const ANGLE_NE: Angle = PI + HALF_PI + QUARTER_PI;
}


pub fn pick_int(n: IScalar) -> IScalar {
  if n <= 0 {
    return 0;
  }

  let mut rng = rand::thread_rng();
  rng.gen_range(0, n)
}

pub fn pick_float(n: FScalar) -> FScalar {
  if n <= 0. {
    return 0.;
  }

  let mut rng = rand::thread_rng();
  rng.gen_range(0., n)
}

pub fn normalize_angle(angle: Angle) -> Angle {
  let mut angle = angle;

  loop {
    if angle < 0. {
      angle = TWO_PI + angle;
    } else if angle >= TWO_PI {
      angle -= TWO_PI;
    } else {
      break;
    }
  }

  angle
}

pub fn normalize_angle_pi(angle: Angle) -> Angle {
  let mut angle = normalize_angle(angle);

  if angle > PI {
    angle -= TWO_PI;
  }

  angle
}

pub fn normalized_angle_diff(angle1: Angle, angle2: Angle) -> Angle {
  normalize_angle_pi(angle1 - angle2)
}

pub fn vec_angle(vector: Vec3f) -> Angle {
  vector.y.atan2(vector.x)
}

pub fn fuzzy_float_eq(num1: f64, num2: f64) -> bool {
  (num1 - num2).abs() < 0.00001
}

pub fn fmin(num1: f64, num2: f64) -> f64 {
  if num1 < num2 {
    num1
  } else {
    num2
  }
}

pub fn fmax(num1: f64, num2: f64) -> f64 {
  if num1 > num2 {
    num1
  } else {
    num2
  }
}

pub fn iscale_floor(scale: FScalar, x: IScalar) -> IScalar {
  (x as FScalar * scale) as IScalar
}

pub fn iscale_ceil(scale: FScalar, x: IScalar) -> IScalar {
  (x as FScalar * scale).ceil() as IScalar
}

pub fn interpolate<T: Add<Output=T>+Sub<Output=T>+Mul<Output=T>+Div<Output=T>+PartialEq+Copy>(out_min: T, out_max: T, in_min: T, in_max: T, value: T) -> T {
  let zero = out_max - out_max;

  let out_range = out_max - out_min;
  let in_range = in_max - in_min;
  if out_range == zero || in_range == zero {
    out_min
  } else {
    out_min + (out_range * (value - in_min) / in_range)
  }
}

pub fn interpolate_vec<T: Add<Output=T>+Sub<Output=T>+Mul<Output=T>+Div<Output=T>+PartialEq+Copy+Default+Into<f64>>(out_min: Vec3<T>, out_max: Vec3<T>, in_min: T, in_max: T, value: T) -> Vec3<T> {
  let x = interpolate(out_min.x, out_max.x, in_min, in_max, value);
  let y = interpolate(out_min.y, out_max.y, in_min, in_max, value);

  let mut z = out_min.z;
  if out_min.z != out_max.z {
    z = interpolate(out_min.z, out_max.z, in_min, in_max, value);
  }

  Vec3::new3(x, y, z)
}

pub fn get_angle_in_clip(angle: Angle, length: usize) -> usize {
  ((normalize_angle(angle) / TWO_PI) * length as Angle) as usize
}

pub fn get_frame_in_clip(time: Time, total_length: usize, length: usize) -> usize {
  //(time as usize / total_length) % length
  (time as usize * length) / total_length % length
}
