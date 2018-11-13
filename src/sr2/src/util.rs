use rand::Rng;
use opensrme_common::*;

pub const PI: f64 = std::f64::consts::PI;
pub const HALF_PI: f64 = PI / 2.;
pub const TWO_PI: f64 = PI * 2.;

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
