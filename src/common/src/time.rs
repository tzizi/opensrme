use super::types::Time;

static mut INSTANT: Option<std::time::Instant> = None;

pub fn instant_init() {
  unsafe {
    INSTANT = Some(std::time::Instant::now());
  }
}

pub fn instant_get_millis() -> Time {
  unsafe {
    if let Some(instant_uw) = INSTANT {
      let diff = instant_uw.elapsed();
      let secs = diff.as_secs();
      let millis = diff.subsec_millis();

      ((secs as Time) * 1000) + (millis as Time)
    } else {
      0
    }
  }
}
