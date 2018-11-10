use types::*;

pub fn calc_route_distance(route: &mut Route) {
  let mut current_distance = 0.;

  for i in 1..route.parts.len() {
    current_distance += (route.parts[i].pos - route.parts[i - 1].pos).len2();

    route.parts[i].distance = current_distance;
  }
}
