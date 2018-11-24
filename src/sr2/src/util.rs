use rand::Rng;
use opensrme_common::*;
use super::*;

pub const PI: Angle = std::f64::consts::PI;
pub const HALF_PI: Angle = PI / 2.;
pub const TWO_PI: Angle = PI * 2.;

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

  return angle;
}

pub fn get_angle_in_clip(angle: Angle, length: usize) -> usize {
  ((normalize_angle(angle) / TWO_PI) * length as Angle) as usize
}

pub fn get_frame_in_clip(time: Time, total_length: usize, length: usize) -> usize {
  //(time as usize / total_length) % length
  (time as usize * length) / total_length % length
}
