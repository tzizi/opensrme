use super::*;

#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct RoutePart {
  // 0, 1, 2
  pub pos: Vec3f,
  // 3
  pub distance: f64,
  pub unk1: u8
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Route {
  pub parts: Vec<RoutePart>
}

impl Route {
  pub fn set_distances(&mut self) {
    let mut current_distance = 0.;

    for i in 1..self.parts.len() {
      current_distance += (self.parts[i].pos - self.parts[i - 1].pos).len2();

      self.parts[i].distance = current_distance;
    }
  }

  fn get_part_id_for_position(&self, position: FScalar) -> Option<usize> {
    for i in 1..self.parts.len() {
      if position <= self.parts[i].distance {
        return Some(i);
      }
    }

    None
  }

  pub fn get_point(&self, position: FScalar) -> Option<Vec3f> {
    if let Some(part_id) = self.get_part_id_for_position(position) {
      Some(util::interpolate_vec(self.parts[part_id - 1].pos, self.parts[part_id].pos,
                                 self.parts[part_id - 1].distance, self.parts[part_id].distance,
                                 position))
    } else {
      None
    }
  }

  pub fn get_angle_at_point(&self, position: FScalar) -> Option<Angle> {
    if let Some(part_id) = self.get_part_id_for_position(position) {
      Some(util::vec_angle(self.parts[part_id].pos -self.parts[part_id - 1].pos))
    } else {
      None
    }
  }

  pub fn get_total_distance(&self) -> FScalar {
    self.parts[self.parts.len() - 1].distance
  }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct RouteData {
  pub route: Option<Route>,
  pub routeid: Option<RouteId>,
  pub threshold: FScalar,
  pub progress: FScalar,
}

impl RouteData {
  pub fn set_route(&mut self, route: Route, threshold: FScalar) {
    self.route = Some(route);
    self.threshold = threshold;
    self.progress = 0.;
  }

  pub fn set_route_to_routeid(&mut self) -> bool {
    if let Some(routeid) = self.routeid {
      let game = globals::get_game();
      self.set_route(game.level.routes[routeid as usize].clone(), 0.);
      true
    } else {
      false
    }
  }

  pub fn reset_route(&mut self, _do_events: bool) {
    self.route = None;
    // TODO: do event
  }

  fn get_total_distance(&self) -> Option<FScalar> {
    if let Some(ref route) = self.route {
      Some(route.get_total_distance())
    } else {
      None
    }
  }

  pub fn step(&mut self, speed: FScalar, delta: Time) -> Option<(Vec3f, Angle)> {
    // TODO: return None if flag 0x4000 is not 0
    if let Some(ref route) = self.route {
      if (route.get_total_distance() - self.progress) > self.threshold {
        self.progress += speed * (delta as FScalar / 1000.);
        if let Some(angle) = route.get_angle_at_point(self.progress) {
          return Some((
            route.get_point(self.progress).unwrap(),
            angle
          ));
        }
      }
    } else {
      return None;
    }

    self.reset_route(true);
    None
  }
}
