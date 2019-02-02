use super::*;
use util::angles::*;
use entity::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TrafficLight {
  H_GREEN  = 0,
  H_YELLOW = 1,
  V_GREEN  = 2,
  V_YELLOW = 3
}

impl TrafficLight {
  fn index(&self) -> usize {
    match *self {
      TrafficLight::H_GREEN  => 0,
      TrafficLight::H_YELLOW => 0,
      TrafficLight::V_GREEN  => 1,
      TrafficLight::V_YELLOW => 1
    }
  }

  fn is_green(&self) -> bool {
    match *self {
      TrafficLight::H_GREEN  => true,
      TrafficLight::H_YELLOW => false,
      TrafficLight::V_GREEN  => true,
      TrafficLight::V_YELLOW => false
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
      trafficlight: TrafficLight::H_GREEN
    }
  }

  pub fn step(&mut self) {
    let context = globals::get_context();

    let state_number = (context.time / 3000) % 4;
    self.trafficlight = match state_number {
      0 => TrafficLight::H_GREEN,
      1 => TrafficLight::H_YELLOW,
      2 => TrafficLight::V_GREEN,
      3 => TrafficLight::V_YELLOW,

      _ => TrafficLight::H_GREEN // shouldn't happen
    };
  }
}

fn is_road(tiledata: LevelTileData) -> bool {
  tiledata >= 10 && tiledata <= 29
}

fn is_one_way_road(tiledata: LevelTileData) -> bool {
  tiledata >= 10 && tiledata <= 17
}

fn is_intersection(tiledata: LevelTileData) -> bool {
  tiledata >= 18 && tiledata <= 29
}

const road_direction_table: [[Angle; 2]; 20] = [
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
  if tiledata < 10 || (tiledata as usize) - 10 >= road_direction_table.len() {
    None
  } else {
    Some(road_direction_table[(tiledata - 10) as usize][trafficlight.index()])
  }
}

//                            e      w      s      n      se     sw     ne     nw
const e_rule: [bool; 8] = [ true,  false, false, false, true,  false, true,  false ];
const w_rule: [bool; 8] = [ false, true,  false, false, false, true,  false, true  ];
const n_rule: [bool; 8] = [ false, false, false, true,  false, false, true,  true  ];
const s_rule: [bool; 8] = [ false, false, true,  false, true,  true,  false, false ];

const road_rules: [[[bool; 8]; 2]; 12] = [
  // 18
  [ e_rule, n_rule ],
  // 19
  [ w_rule, n_rule ],
  // 20
  [ e_rule, s_rule ],
  // 21
  [ w_rule, s_rule ],
  // 22
  [ e_rule, s_rule ],
  // 23
  [ e_rule, n_rule ],
  // 24
  [ w_rule, s_rule ],
  // 25
  [ w_rule, n_rule ],
  // 26
  [ e_rule, s_rule ],
  // 27
  [ w_rule, s_rule ],
  // 28
  [ e_rule, n_rule ],
  // 29
  [ w_rule, n_rule ]
];

fn can_move_on_road(tiledata: LevelTileData, last_tiledata: LevelTileData, trafficlight: TrafficLight) -> bool {
  if !is_road(last_tiledata) {
    return true;
  }

  if is_one_way_road(tiledata) {
    return true;
  }

  if !is_one_way_road(last_tiledata) {
    return true;
  }

  if !trafficlight.is_green() {
    return false;
  }

  let roaddata: usize = tiledata as usize - 10 - 8;
  let last_roaddata: usize = last_tiledata as usize - 10;

  road_rules[roaddata][trafficlight.index()][last_roaddata]
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

  let lane = get_lane(tilepos);
  if let Some(lane) = lane {
    if lane.x <= 0 && lane.y <= 0 {
      //return None;
      return Some(tilepos_to_pos(tilepos));
    }

    let mut tilepos = tilepos;

    if lane.x > 0 {
      tilepos.x = tilepos.x + 1;
    }

    if lane.y > 0 {
      tilepos.y = tilepos.y + 1;
    }

    Some(tilepos_to_pos(tilepos))
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
    util::interpolate_vec(entity.pos, road_middle, 0., diff_len, util::fmin(diff_len, 10. * (delta as FScalar / 1000.)))
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

    if entity.stance == EntityStance::Running || true {
      let game = globals::get_game();

      let tiledata = level::get_tiledata_for_pos(&game.level, entity.pos);

      let road_direction = get_road_direction(tiledata, game.vehicle_state.trafficlight);
      if let Some(road_direction) = road_direction {
        if can_move_on_road(tiledata, self.last_tiledata, game.vehicle_state.trafficlight) {
          self.wanted_speed = 25.;

          let turn_amount = get_turn_amount(entity, delta, road_direction);
          if turn_amount == 0. {
            entity.angle = road_direction;
            self.wanted_speed = 50.;
          } else {
            entity.angle += turn_amount;
          }

          self.last_tiledata = tiledata;
        }
      } else if let Some(angle) = find_angle_to_road(level::pos_to_tilepos(entity.pos)) {
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
      let context = get_context();
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
    if let Some(pos) = get_road_spawn(pos) {
      entity.pos = pos;
    } else {
      return None;
    }

    let game = globals::get_game();
    let tiledata = level::get_tiledata_for_pos(&game.level, entity.pos);

    if let Some(direction) = get_road_direction(tiledata, game.vehicle_state.trafficlight) {
      entity.angle = direction;
    }

    entity.hidden = false;

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
