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
  Shooting = 5,
  // 6, also shooting?
  Stance6 = 6,
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

#[derive(Debug, Clone)]
pub struct Entity {
  pub class: ClassId,
  pub entity_type: EntityType,
  pub pos: Vec3f,
  pub angle: Angle,
  pub stance: EntityStance
}

impl Entity {
  pub fn new(class: ClassId) -> Self {
    let context = get_context();

    Entity {
      class,
      entity_type: get_entitytype(context.data.classes[class as usize].entity_type),
      pos: Vec3f::new2(0., 0.),
      angle: 0.,
      stance: EntityStance::Standing
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
}



pub fn draw_person(entity: &mut Entity) {
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

pub fn draw_vehicle(entity: &mut Entity) {
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
