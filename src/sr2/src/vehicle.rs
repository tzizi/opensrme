use super::*;
use util::angles::*;
use entity::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TrafficLight {
  HGreen  = 0,
  HYellow = 1,
  VGreen  = 2,
  VYellow = 3
}

impl TrafficLight {
  fn index(&self) -> usize {
    match *self {
      TrafficLight::HGreen  => 0,
      TrafficLight::HYellow => 0,
      TrafficLight::VGreen  => 1,
      TrafficLight::VYellow => 1
    }
  }

  fn is_green(&self) -> bool {
    match *self {
      TrafficLight::HGreen  => true,
      TrafficLight::HYellow => false,
      TrafficLight::VGreen  => true,
      TrafficLight::VYellow => false
    }
  }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VehicleState {
  trafficlight: TrafficLight
}

impl VehicleState {
  pub fn new() -> Self {
    VehicleState {
      trafficlight: TrafficLight::HGreen
    }
  }

  pub fn step(&mut self) {
    let context = globals::get_context();

    let state_number = (context.time / 3000) % 4;
    self.trafficlight = match state_number {
      0 => TrafficLight::HGreen,
      1 => TrafficLight::HYellow,
      2 => TrafficLight::VGreen,
      3 => TrafficLight::VYellow,

      _ => TrafficLight::HGreen // shouldn't happen
    };
  }
}

fn is_road(tiledata: LevelTileData) -> bool {
  tiledata >= 10 && tiledata <= 29
}

fn is_road_or_0(tiledata: LevelTileData) -> bool {
  is_road(tiledata) || tiledata == 0
}

fn is_one_way_road(tiledata: LevelTileData) -> bool {
  tiledata >= 10 && tiledata <= 17
}

fn is_intersection(tiledata: LevelTileData) -> bool {
  tiledata >= 18 && tiledata <= 29
}

const ROAD_DIRECTION_TABLE: [[Angle; 2]; 20] = [
  // 10
  [ANGLE_E, ANGLE_E],
  // 11
  [ANGLE_W, ANGLE_W],
  // 12
  [ANGLE_S, ANGLE_S],
  // 13
  [ANGLE_N, ANGLE_N],
  // 14
  [ANGLE_SE, ANGLE_SE],
  // 15
  [ANGLE_SW, ANGLE_SW],
  // 16
  [ANGLE_NE, ANGLE_NE],
  // 17
  [ANGLE_NW, ANGLE_NW],
  // 18
  [ANGLE_E, ANGLE_N],
  // 19
  [ANGLE_W, ANGLE_N],
  // 20
  [ANGLE_E, ANGLE_S],
  // 21
  [ANGLE_W, ANGLE_S],
  // 22
  [ANGLE_E, ANGLE_SE],
  // 23
  [ANGLE_E, ANGLE_NE],
  // 24
  [ANGLE_W, ANGLE_SW],
  // 25
  [ANGLE_W, ANGLE_NW],
  // 26
  [ANGLE_SE, ANGLE_S],
  // 27
  [ANGLE_SW, ANGLE_S],
  // 28
  [ANGLE_NE, ANGLE_N],
  // 29
  [ANGLE_NW, ANGLE_N]
];

fn get_road_direction(tiledata: LevelTileData, trafficlight: TrafficLight) -> Option<Angle> {
  if tiledata < 10 || (tiledata as usize) - 10 >= ROAD_DIRECTION_TABLE.len() {
    None
  } else {
    Some(ROAD_DIRECTION_TABLE[(tiledata - 10) as usize][trafficlight.index()])
  }
}

//                            e      w      s      n      se     sw     ne     nw
const E_RULE: [bool; 8] = [ true,  false, false, false, true,  false, true,  false ];
const W_RULE: [bool; 8] = [ false, true,  false, false, false, true,  false, true  ];
const N_RULE: [bool; 8] = [ false, false, false, true,  false, false, true,  true  ];
const S_RULE: [bool; 8] = [ false, false, true,  false, true,  true,  false, false ];

const ROAD_RULES: [[[bool; 8]; 2]; 12] = [
  // 18
  [ E_RULE, N_RULE ],
  // 19
  [ W_RULE, N_RULE ],
  // 20
  [ E_RULE, S_RULE ],
  // 21
  [ W_RULE, S_RULE ],
  // 22
  [ E_RULE, S_RULE ],
  // 23
  [ E_RULE, N_RULE ],
  // 24
  [ W_RULE, S_RULE ],
  // 25
  [ W_RULE, N_RULE ],
  // 26
  [ E_RULE, S_RULE ],
  // 27
  [ W_RULE, S_RULE ],
  // 28
  [ E_RULE, N_RULE ],
  // 29
  [ W_RULE, N_RULE ]
];

// TODO: split into multiple functions like in original game
fn can_move_on_road(entity: &EntityBase, angle: Angle, last_tiledata: LevelTileData, trafficlight: TrafficLight) -> (bool, i8) {
  let amount = 3 * (entity.get_class().width as IScalar) >> 1;
  let wanted_pos = entity.pos + util::cossin(angle) * (amount as FScalar);
  let level = &globals::get_game().level;
  let wanted_tiledata = level::get_tiledata_for_pos(level, wanted_pos);

  if !is_point_free(entity, wanted_pos) {
    return (false, wanted_tiledata);
  }

  if !is_road(last_tiledata) {
    return (true, wanted_tiledata);
  }

  if is_one_way_road(wanted_tiledata) {
    return (true, wanted_tiledata);
  }

  if is_intersection(last_tiledata) {
    return (true, wanted_tiledata);
  }

  if !trafficlight.is_green() {
    return (false, wanted_tiledata);
  }

  if !is_road(wanted_tiledata) {
    return (true, wanted_tiledata);
  }

  let wanted_roaddata: usize = wanted_tiledata as usize - 10 - 8;
  let last_roaddata: usize = last_tiledata as usize - 10;

  if !ROAD_RULES[wanted_roaddata][trafficlight.index()][last_roaddata] {
    return (false, wanted_tiledata);
  }

  (true, wanted_tiledata)
}

fn get_lane(tilepos: Vec3i) -> Option<Vec3i> {
  let level = &globals::get_game().level;

  let tiledata = get_tiledata_for_tilepos(level, tilepos);
  if tiledata < 10 || tiledata > 13 {
    return None;
  }

  let mut x_direction = 0;
  let mut y_direction = 0;

  if tiledata == 10 || tiledata == 11 {
    // E/W
    if tiledata == get_tiledata_for_tilepos(level, Vec3i::new2(tilepos.x, tilepos.y - 1)) {
      y_direction = -1;
    }

    if tiledata == get_tiledata_for_tilepos(level, Vec3i::new2(tilepos.x, tilepos.y + 1)) {
      y_direction = 1;
    }
  } else if tiledata == 12 || tiledata == 13 {
    // S/N
    if tiledata == get_tiledata_for_tilepos(level, Vec3i::new2(tilepos.x - 1, tilepos.y)) {
      x_direction = -1;
    }

    if tiledata == get_tiledata_for_tilepos(level, Vec3i::new2(tilepos.x + 1, tilepos.y)) {
      x_direction = 1;
    }
  }

  Some(Vec3i::new2(x_direction, y_direction))
}

fn get_road_middle(pos: Vec3f) -> Option<Vec3f> {
  let tilepos = pos_to_tilepos(pos);
  let pos_tilepos = tilepos_to_pos(tilepos);

  let lane = get_lane(tilepos);
  if let Some(lane) = lane {
    /*if lane.x <= 0 && lane.y <= 0 {
      //return None;
      return Some(tilepos_to_pos(tilepos));
    }*/

    let mut newpos = pos;

    if lane.x > 0 {
      newpos.x = pos_tilepos.x + util::TILESIZE as FScalar;
    } else if lane.x < 0 {
      newpos.x = pos_tilepos.x;
    }

    if lane.y > 0 {
      newpos.y = newpos.y + util::TILESIZE as FScalar;
    } else if lane.y < 0 {
      newpos.y = pos_tilepos.y;
    }

    Some(newpos)
  } else {
    None
  }
}

fn get_turn_amount(entity: &EntityBase, delta: Time, wanted_angle: Angle) -> Angle {
  let diff = util::normalized_angle_diff(entity.angle, wanted_angle);

  if util::fuzzy_float_eq(diff, 0.) {
    0.
  } else if diff < 0. {
    util::fmin(-diff, 4. * (delta as Angle/1000.))
  } else {
    -util::fmin(diff, 4. * (delta as Angle/1000.))
  }
}

fn move_to_middle_of_road(entity: &EntityBase, delta: Time) -> Vec3f {
  let road_middle = get_road_middle(entity.pos);
  if let Some(road_middle) = road_middle {
    //println!("{:?}", road_middle - entity.pos);
    let diff = road_middle - entity.pos;
    let diff_len = diff.len2();
    let result = util::interpolate_vec(entity.pos, road_middle, 0., diff_len, util::fmin(diff_len, 10. * (delta as FScalar / 1000.)));

    if (result - road_middle).len2() < 1. {
      road_middle
    } else {
      result
    }
  } else {
    entity.pos
  }
}

fn get_road_spawn(pos: Vec3f) -> Option<Vec3f> {
  let game = globals::get_game();
  let tiledata = level::get_tiledata_for_pos(&game.level, pos);

  if tiledata >= 10 && tiledata <= 13 {
    if game.entity_spawn_counter % 4 == tiledata as usize - 10 {
      return get_road_middle(pos);
    }
  }

  None
}

fn is_point_free(self_entity: &EntityBase, point: Vec3f) -> bool {
  let game = globals::get_game();

  for entity in game.entities.iter() {
    if entity.base.id != self_entity.id && !entity.base.hidden {
      if (entity.base.entity_type == EntityType::Type8 ||
          entity.base.entity_type == EntityType::PlayerVehicle ||
          entity.base.entity_type == EntityType::MovingVehicle ||
          entity.base.entity_type == EntityType::PoliceCar) {
        if (point - entity.base.pos).abs().max2() < entity.get_class().width {
          return false;
        }
      } else if entity.base.entity_type == EntityType::Player {
        if (point - entity.base.pos).abs().max2() < self_entity.get_class().height {
          return false;
        }
      }
    }
  }

  true
}

// not in the original game
// TODO:
//   start with the tile closest to the angle of the car
//   find all nearest blocks first
fn find_angle_to_road(tilepos: Vec3i) -> Option<Angle> {
  let game = globals::get_game();

  for sign_x in 0..2 {
    for search_x in 0..2 {
      let mut tile_x = search_x + 1;
      if sign_x == 0 {
        tile_x = -tile_x;
      }

      for sign_y in 0..2 {
        for search_y in 0..2 {
          let mut tile_y = search_y + 1;
          if sign_y == 0 {
            tile_y = -tile_y;
          }

          let mut pos = tilepos;
          pos.x += tile_x;
          pos.y += tile_y;

          let tiledata = level::get_tiledata_for_tilepos(&game.level, pos.into());
          if is_road(tiledata) {
            return Some(util::vec_angle((pos - tilepos).into()))
          }
        }
      }
    }
  }

  None
}

#[derive(Debug, Clone, PartialEq)]
pub struct VehicleData {
  last_tiledata: LevelTileData,
  wanted_speed: FScalar
}

impl VehicleData {
  pub fn new() -> Self {
    VehicleData {
      last_tiledata: -1,
      wanted_speed: 0.
    }
  }

  fn step_drive_along_road(&mut self, entity: &mut EntityBase, delta: Time) {
    entity.update_prev();

    self.wanted_speed = 0.;

    if entity.stance == EntityStance::Running {
      let game = globals::get_game();

      let tiledata = level::get_tiledata_for_pos(&game.level, entity.pos);

      let road_direction = get_road_direction(tiledata, game.vehicle_state.trafficlight);
      if let Some(road_direction) = road_direction {
        let (can_move, new_tiledata) = can_move_on_road(entity, road_direction, self.last_tiledata, game.vehicle_state.trafficlight);
        if can_move {
          self.wanted_speed = 25.;

          let turn_amount = get_turn_amount(entity, delta, road_direction);
          if turn_amount == 0. {
            entity.angle = road_direction;
            self.wanted_speed = 50.;
          } else {
            entity.angle += turn_amount;
          }

          self.last_tiledata = new_tiledata;
        }
      } else if let Some(angle) = find_angle_to_road(level::pos_to_tilepos(entity.pos)) {
        // TODO: merge these "turn amount" functions to avoid code duplication with above
        let turn_amount = get_turn_amount(entity, delta, angle);
        if turn_amount != 0. {
          self.wanted_speed = 25.;
          entity.angle += turn_amount;
        } else {
          self.wanted_speed = 50.;
        }
      }

      entity.pos = move_to_middle_of_road(entity, delta);
    }

    self.move_vehicle(entity, delta);
  }

  fn accelerate_to_wanted_speed(&self, entity: &mut EntityBase, delta: Time) {
    if entity.speed > self.wanted_speed {
      entity.speed = util::fmax(self.wanted_speed, entity.speed - 200. * (delta as FScalar / 1000.));
    } else {
      entity.speed = util::fmin(self.wanted_speed, entity.speed + 100. * (delta as FScalar / 1000.));
    }

    if util::fuzzy_float_eq(entity.speed, 0.) {
      entity.speed = 0.;
    }
  }

  fn move_vehicle(&mut self, entity: &mut EntityBase, delta: Time) {
    self.accelerate_to_wanted_speed(entity, delta);
    if entity.speed == 0. {
      return;
    }

    entity.move_forward(delta);
    // TODO: calculate skidmarks
  }

  fn init_simple_vehicle(&mut self, entity: &mut EntityBase) {
    // motorcycle
    if entity.class == 38 || entity.class == 39 {
      entity.gender = EntityBase::pick_gender();
      entity.palette = EntityBase::pick_npc_person_palette();
    } else {
      entity.palette = util::pick_int(2) + 1;
    }
  }

  fn draw_basic_vehicle(&self, entity: &EntityBase, palette: PaletteId) {
    let context = globals::get_context();
    let class = entity.get_class();

    let clip = &context.data.clips[class.clip as usize];
    let clip_angle = &clip[util::get_angle_in_clip(entity.angle, clip.len())];
    let current_sprite = clip_angle[0];

    let imageid = sprite::get_image_from_sprite(current_sprite).unwrap();
    sprite::draw_sprite_palette(current_sprite, entity.pos.into(), 0, &vec![(imageid, palette)]);
  }

  fn draw_motorcycle(&self, entity: &EntityBase) {
    self.draw_basic_vehicle(entity, 0);

    // TODO: change gender if image with palette is loaded?
    if entity.stance != EntityStance::Standing || true {
      let context = globals::get_context();
      let clip = &context.data.clips[entity.gender.get_clip_id() as usize];
      let clip_angle = &clip[util::get_angle_in_clip(entity.angle, clip.len())];
      let current_sprite = clip_angle[0];

      let imageid = sprite::get_image_from_sprite(current_sprite).unwrap();
      sprite::draw_sprite_palette(current_sprite, entity.pos.into(), 0, &vec![(imageid, entity.palette)]);
    }
  }
}

impl EntityData for VehicleData {
  fn init(&mut self, entity: &mut EntityBase) {
    if (entity.entity_type == EntityType::Type8 ||
        entity.entity_type == EntityType::MovingVehicle) {
      self.init_simple_vehicle(entity);
    }

    if entity.entity_type == EntityType::MovingVehicle {
      entity.hidden = true;
    }
  }

  fn get_collision_info(&self, entity: &EntityBase) -> Option<collision::ShapeInfo> {
    if (entity.entity_type == EntityType::Type8 ||
        entity.entity_type == EntityType::PlayerVehicle ||
        entity.entity_type == EntityType::MovingVehicle ||
        entity.entity_type == EntityType::EnemyVehicle ||
        entity.entity_type == EntityType::PoliceCar) {
      let context = globals::get_context();
      let class = &context.data.classes[entity.class as usize];

      Some(collision::ShapeInfo {
        shape: collision::Shape::Rect(Vec3i::new2(class.width as i32, class.height as i32)),
        weight: class.weight
      })
    } else {
      None
    }
  }

  fn spawn(&mut self, entity: &mut EntityBase, pos: Vec3f) -> Option<Vec3f> {
    if !is_point_free(entity, pos) {
      return None;
    }

    if let Some(pos) = get_road_spawn(pos) {
      entity.pos = pos;
    } else {
      return None;
    }

    let game = globals::get_game();
    let tiledata = level::get_tiledata_for_pos(&game.level, entity.pos);

    entity.pos = move_to_middle_of_road(entity, 1000);

    if let Some(direction) = get_road_direction(tiledata, game.vehicle_state.trafficlight) {
      entity.angle = direction;
    }

    self.last_tiledata = tiledata;
    if !can_move_on_road(entity, entity.angle, tiledata, game.vehicle_state.trafficlight).0 {
      return None;
    }

    entity.hidden = false;
    entity.stance = EntityStance::Running;

    Some(entity.pos)
  }

  fn step(&mut self, entity: &mut EntityBase, delta: Time) {
    self.step_drive_along_road(entity, delta);
  }

  fn draw(&self, entity: &EntityBase) {
    if entity.class == 38 || entity.class == 39 {
      return self.draw_motorcycle(entity);
    }

    // TODO: if broken (flags 0x2) then draw broken car
    self.draw_basic_vehicle(entity, entity.palette);
  }
}
