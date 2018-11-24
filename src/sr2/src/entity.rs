use super::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum EntityType {
  Unknown = 0,
  // 1, pedestrian, can follow path
  Type1 = 1,
  // 2
  Player = 2,
  // 3
  Pedestrian = 3,
  // 4 a pedestrian that spawned out of a vehicle
  VehiclePedestrian = 4,
  // 5
  Gangster = 5,
  // 6
  Police = 6,
  // 7 related to a gangster
  Type7 = 7,
  // 8, vehicle
  Type8 = 8,
  // 9
  DrivenVehicle = 9,
  // 10
  MovingVehicle = 10,
  // 11
  EnemyVehicle = 11,
  // 12
  PoliceCar = 12,
  // 13
  Pickup = 13,
  // 14
  Type14 = 14,
  // 15
  Type15 = 15,
  // 16
  GarageDoor = 16,
  // 17
  Type17 = 17,
  // 18
  Type18 = 18,
  // 19
  Type19 = 19,
  // 20 (sports car?)
  Type20 = 20
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum EntityStance {
  // 0
  Standing = 0,
  // 1
  Walking = 1,
  // 2
  Running = 2,
  // 3
  Dead = 3,
  // 4
  LyingDown = 4,
  // 5
  Aiming = 5,
  // 6
  Shooting = 6,
  // 7
  Punching = 7,
  // 8
  Riding = 8,
  // 9
  Sliding = 9,
  // 10
  Unknown = 10
}

pub fn get_entitytype(number: i32) -> EntityType {
  match number {
    1 => EntityType::Type1,
    2 => EntityType::Player,
    3 => EntityType::Pedestrian,
    4 => EntityType::VehiclePedestrian,
    5 => EntityType::Gangster,
    6 => EntityType::Police,
    7 => EntityType::Type7,
    8 => EntityType::Type8,
    9 => EntityType::DrivenVehicle,
    10 => EntityType::MovingVehicle,
    11 => EntityType::EnemyVehicle,
    12 => EntityType::PoliceCar,
    13 => EntityType::Pickup,
    14 => EntityType::Type14,
    15 => EntityType::Type15,
    16 => EntityType::GarageDoor,
    17 => EntityType::Type17,
    18 => EntityType::Type18,
    19 => EntityType::Type19,
    20 => EntityType::Type20,
    _  => EntityType::Unknown
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Entity {
  pub class: ClassId,
  pub entity_type: EntityType,
  pub pos: Vec3f,
  pub angle: Angle,
  pub prev_pos: Vec3f,
  pub prev_angle: Angle,
  pub walking_direction: Vec3f,
  pub walking_angle: Angle,
  pub stance: EntityStance,
  pub stance_millis: Time,
  pub speed: FScalar
}

impl Entity {
  pub fn new(class: ClassId) -> Self {
    let context = get_context();

    Entity {
      class,
      entity_type: get_entitytype(context.data.classes[class as usize].entity_type),
      pos: Vec3f::new2(0., 0.),
      angle: 0.,
      prev_pos: Vec3f::new2(0., 0.),
      prev_angle: 0.,
      walking_direction: Vec3f::new2(0., 0.),
      walking_angle: 0.,
      stance: EntityStance::Standing,
      stance_millis: 0,
      speed: 0.
    }
  }

  pub fn get_class(&self) -> &EntityClass {
    &globals::get_context().data.classes[self.class as usize]
  }

  pub fn draw(&mut self) {
    // TODO: check if hidden

    let entity_typeid = self.entity_type as i32;
    if entity_typeid >= 1 && entity_typeid <= 7 {
      return draw_person(self);
    } else if entity_typeid >= 8 && entity_typeid <= 11 {
      return draw_vehicle(self);
    }
  }

  pub fn step(&mut self, delta: Time) {
    // TODO: check if hidden

    self.stance_millis += delta;

    match self.entity_type {
      EntityType::Type1 => {
        step_person(self);
      },
      EntityType::Pedestrian => {
        step_sidewalk_path(self, delta);
      },
      EntityType::VehiclePedestrian => {
        step_sidewalk_path(self, delta);
      },
      _ => {}
    }
  }
}


fn update_prev(entity: &mut Entity) {
  entity.prev_pos = entity.pos;
  entity.prev_angle = entity.angle;
}

fn set_new_stance(entity: &mut Entity, newstance: EntityStance) {
  if entity.stance == newstance {
    return;
  }

  entity.stance = newstance;
  entity.stance_millis = 0;
}

fn step_person(entity: &mut Entity) -> bool {
  if entity.stance == EntityStance::Dead {
    return true;
  }

  update_prev(entity);

  match entity.stance {
    EntityStance::Punching => {
      // TODO
    },
    EntityStance::Shooting => {
      // TODO
    },
    EntityStance::Aiming => {
      if entity.stance_millis > 1000 {
        set_new_stance(entity, EntityStance::Standing)
      }
    },
    EntityStance::LyingDown => {
      if entity.stance_millis > 3000 {
        set_new_stance(entity, EntityStance::Standing)
      }

      return true;
    },
    _ => {}
  }

  // TODO: step route
  return false;
}

fn pick_sidewalk_direction(entity: &mut Entity) {
  let angle: Angle = util::pick_int(4) as f64 * util::HALF_PI;
  let class = entity.get_class().clone();
  entity.walking_direction.x = class.width * angle.cos();
  entity.walking_direction.y = class.height * angle.sin();
  entity.walking_angle = angle;
}

fn move_forward(entity: &mut Entity, delta: Time, speed: FScalar) {
  if speed == 0. {
    return;
  }

  let amount = (delta as FScalar) / 1000. * speed;
  entity.pos.x += amount * entity.angle.cos();
  entity.pos.y += amount * entity.angle.sin();
}

fn step_sidewalk_path(entity: &mut Entity, delta: Time) -> bool {
  if step_person(entity) {
    return true;
  }

  let context = globals::get_context();

  match entity.stance {
    EntityStance::Standing => {
      entity.speed = 15. + util::pick_float(15.);
      pick_sidewalk_direction(entity);
      set_new_stance(entity, EntityStance::Walking);
    },
    EntityStance::Walking => {
      let old_angle = entity.angle;

      entity.angle = entity.walking_angle;

      let speed = entity.speed;
      move_forward(entity, delta, speed);

      if !level::pos_is_sidewalk(&context.game.level,
                                 entity.pos + entity.walking_direction) {
        entity.pos.x = entity.prev_pos.x;
        entity.pos.y = entity.prev_pos.y;
        entity.angle = old_angle;
        set_new_stance(entity, EntityStance::Standing);
      }
    },
    _ => {}
  }

  return false;
}

fn draw_person(entity: &mut Entity) {
  if entity.stance == EntityStance::Riding {
    return;
  }

  let mut context = globals::get_context();
  let class = entity.get_class();

  let mut stance = entity.stance;
  if stance == EntityStance::Unknown {
    stance = EntityStance::Running;
  }

  let clip = &context.data.clips[(class.clip + stance as i32) as usize];
  // TODO: get the correct index from the angle
  let clip_angle = &clip[0];
  // TODO: animate properly
  let current_sprite = clip_angle[0];

  // TODO: palettes
  sprite::draw_sprite(current_sprite, entity.pos.into(), 0);
}

fn draw_vehicle(entity: &mut Entity) {
  if entity.class == 38 || entity.class == 39 {
    // TODO: draw motorcycle
    return;
  }

  // TODO: if broken (flags 0x2) then draw broken car
  // TODO: animate

  let context = globals::get_context();
  let class = entity.get_class();

  let clip = &context.data.clips[class.clip as usize];
  // TODO: palettes
  let clip_angle = &clip[0];
  let current_sprite = clip_angle[0];

  sprite::draw_sprite(current_sprite, entity.pos.into(), 0);
}
