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
  PlayerVehicle = 9,
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

impl EntityType {
  pub fn is_person(&self) -> bool {
    (*self as i32) >= 0 && (*self as i32) <= 7
  }

  pub fn is_vehicle(&self) -> bool {
    ((*self as i32) >= 8 && (*self as i32) <= 12) || (*self as i32) == 20
  }

  pub fn is_npc(&self) -> bool {
    return
      *self == EntityType::Pedestrian ||
      *self == EntityType::VehiclePedestrian ||
      *self == EntityType::Gangster ||
      *self == EntityType::Police ||
      *self == EntityType::MovingVehicle ||
      *self == EntityType::EnemyVehicle ||
      *self == EntityType::PoliceCar;
  }
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
    9 => EntityType::PlayerVehicle,
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
pub struct EntityBase {
  pub class: ClassId,
  pub entity_type: EntityType,
  pub pos: Vec3f,
  pub angle: Angle,
  pub prev_pos: Vec3f,
  pub prev_angle: Angle,
  pub speed: FScalar,

  pub stance: EntityStance,
  pub stance_millis: Time,

  pub palette: PaletteId,

  pub hidden: bool // 0x01
}

impl EntityBase {
  pub fn update_prev(&mut self) {
    self.prev_pos = self.pos;
    self.prev_angle = self.angle;
  }

  pub fn move_forward(&mut self, delta: Time) {
    if self.speed == 0. {
      return;
    }

    let amount = (delta as FScalar) / 1000. * self.speed;
    self.pos.x += amount * self.angle.cos();
    self.pos.y += amount * self.angle.sin();
  }

  pub fn get_class(&self) -> &EntityClass {
    &globals::get_context().data.classes[self.class as usize]
  }

  pub fn set_new_stance(&mut self, newstance: EntityStance) {
    if self.stance == newstance {
      return;
    }

    self.stance = newstance;
    self.stance_millis = 0;
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PersonData {
  pub walking_direction: Vec3f,
  pub walking_angle: Angle
}

pub trait EntityData {
  fn init(&mut self, _entity: &mut EntityBase) {}
  fn step(&mut self, _entity: &mut EntityBase, _delta: Time) {}
  fn draw(&self, _entity: &EntityBase) {}
  fn despawn_action(&mut self, _entity: &mut EntityBase) -> bool { true }
  fn can_spawn_at(&self, _level: &Level, _pos: Vec3f) -> bool { false }
}

struct NullEntityData();
impl EntityData for NullEntityData {}

pub struct Entity {
  pub base: EntityBase,
  pub data: Box<EntityData>
}

fn create_entity_data(entity_type: EntityType) -> Box<EntityData> {
  if entity_type.is_person() {
    return Box::new(person::PersonData::new());
  } else if entity_type.is_vehicle() {
    return Box::new(vehicle::VehicleData::new());
  }

  Box::new(NullEntityData())
}

impl Entity {
  pub fn new(class: ClassId) -> Self {
    let context = get_context();

    let mut base = EntityBase {
      class,
      entity_type: get_entitytype(context.data.classes[class as usize].entity_type),
      pos: Vec3f::new2(0., 0.),
      angle: 0.,
      prev_pos: Vec3f::new2(0., 0.),
      prev_angle: 0.,
      stance: EntityStance::Standing,
      stance_millis: 0,
      palette: 0,
      speed: 0.,
      hidden: false
    };

    let mut data = create_entity_data(base.entity_type);
    data.init(&mut base);

    Entity {
      base,
      data
    }
  }

  pub fn get_class(&self) -> &EntityClass {
    &globals::get_context().data.classes[self.base.class as usize]
  }

  pub fn draw(&self) {
    if self.base.hidden {
      return;
    }

    self.data.draw(&self.base);
  }

  pub fn step(&mut self, delta: Time) {
    if self.base.hidden {
      return;
    }

    self.base.stance_millis += delta;

    self.data.step(&mut self.base, delta);
  }

  pub fn despawn(&mut self) -> bool {
    self.data.despawn_action(&mut self.base)
  }

  pub fn can_spawn_at(&self, pos: Vec3f) -> bool {
    let level = &globals::get_game().level;

    self.data.can_spawn_at(level, pos)
  }
}
