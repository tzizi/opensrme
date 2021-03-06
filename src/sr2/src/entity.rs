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

impl EntityStance {
  pub fn is_self_moving(&self) -> bool {
    (*self == EntityStance::Walking ||
     *self == EntityStance::Running)
  }
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

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum EntityGender {
  // 0
  Female = 0,
  // 1
  Male = 1
}

impl EntityGender {
  pub fn get_clip_id(&self) -> ClipId {
    match *self {
      EntityGender::Female => 18,
      EntityGender::Male   => 8
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct EntityBase {
  pub id: EntityId,
  pub class: ClassId,
  pub entity_type: EntityType,
  pub pos: Vec3f,
  pub angle: Angle,
  pub prev_pos: Vec3f,
  pub prev_angle: Angle,
  pub speed: FScalar,
  pub sort_order: IScalar,

  pub stance: EntityStance,
  pub stance_millis: Time,

  pub palette: PaletteId,
  pub gender: EntityGender,

  pub route: route::RouteData,

  pub hidden: bool,          //    0x01
  pub following_route: bool, // 0x10000
  pub can_update_sort: bool, // 0x20000
}

impl EntityBase {
  pub fn new(id: EntityId, class: ClassId) -> Self {
    let context = globals::get_context();

    EntityBase {
      id,
      class,
      entity_type: get_entitytype(context.data.classes[class as usize].entity_type),
      pos: Vec3f::new2(0., 0.),
      sort_order: 0,
      angle: 0.,
      prev_pos: Vec3f::new2(0., 0.),
      prev_angle: 0.,
      stance: EntityStance::Standing,
      stance_millis: 0,
      palette: 0,
      gender: EntityGender::Female,

      route: route::RouteData::default(),

      speed: 0.,

      hidden: false,
      following_route: false,
      can_update_sort: true
    }
  }

  pub fn init(&mut self) {
    *self = EntityBase::new(self.id, self.class);
  }

  pub fn update_prev(&mut self) {
    self.prev_pos = self.pos;
    self.prev_angle = self.angle;
  }

  pub fn strafe(&mut self, angle: Angle, delta: Time) {
    if self.speed == 0. {
      return;
    }

    let amount = (delta as FScalar) / 1000. * self.speed;
    self.pos.x += amount * (self.angle + angle).cos();
    self.pos.y += amount * (self.angle + angle).sin();

    self.update_pos();
  }

  pub fn move_forward(&mut self, delta: Time) {
    self.strafe(0., delta);
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

  pub fn pick_gender() -> EntityGender {
    match util::pick_int(2) {
      0 => EntityGender::Male,
      _ => EntityGender::Female
    }
  }

  pub fn pick_npc_person_palette() -> PaletteId {
    12 + util::pick_int(1)
  }

  pub fn update_pos(&mut self) {
    if self.can_update_sort {
      self.sort_order = self.pos.y as IScalar;
    }
  }

  pub fn step_route(&mut self, delta: Time) {
    if self.route.route.is_some() {
      if let Some((pos, angle)) = self.route.step(self.speed, delta) {
        //println!("{:?} {:?}", pos, angle);
        self.angle = angle;
        self.pos = pos;
        self.update_pos();
      } else {
        self.following_route = false;
      }
    }
  }
}

pub trait EntityData {
  fn init(&mut self, _entity: &mut EntityBase) {}
  fn get_collision_info(&self, _entity: &EntityBase) -> Option<collision::ShapeInfo> { None }
  fn spawn(&mut self, _entity: &mut EntityBase, _pos: Vec3f) -> Option<Vec3f> { None }
  fn step(&mut self, _entity: &mut EntityBase, _delta: Time) {}
  fn draw(&self, _entity: &EntityBase) {}
  fn despawn_action(&mut self, _entity: &mut EntityBase) -> bool { true }
}

struct NullEntityData();
impl EntityData for NullEntityData {}

pub struct Entity {
  pub base: EntityBase,
  pub collision: Option<collision::PhysicalObject>,
  pub data: Box<EntityData>
}

fn create_entity_data(entity_type: EntityType) -> Box<EntityData> {
  if entity_type.is_person() {
    return Box::new(person::PersonData::new(entity_type));
  } else if entity_type.is_vehicle() {
    return Box::new(vehicle::VehicleData::new());
  }

  Box::new(NullEntityData())
}

impl Entity {
  pub fn new(id: EntityId, class: ClassId) -> Self {
    let base = EntityBase::new(id, class);
    let data = create_entity_data(base.entity_type);

    let mut entity = Entity {
      base,
      collision: None,
      data
    };

    entity.data.init(&mut entity.base);

    entity
  }

  pub fn init(&mut self) {
    self.base.init();
    self.data.init(&mut self.base);
  }

  pub fn after_init(&mut self) {
    if let Some(info) = self.data.get_collision_info(&self.base) {
      self.collision = Some(collision::PhysicalObject::new_from_info(info));
    }
  }

  pub fn spawn(&mut self, pos: Vec3f) -> Option<Vec3f> {
    self.init();

    if let Some(pos) = self.data.spawn(&mut self.base, pos) {
      self.set_pos(pos);
      self.after_init();
      Some(pos)
    } else {
      None
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

  pub fn is_physical(&self) -> bool {
    // TODO: extra checks
    self.collision.is_some() && !self.base.hidden
  }

  pub fn set_pos(&mut self, pos: Vec3f) {
    self.base.pos = pos;
    self.base.update_pos();

    if let Some(ref mut collision) = self.collision {
      collision.update_isometry(&self.base);
    }
  }

  pub fn step(&mut self, delta: Time) {
    if self.base.hidden {
      return;
    }

    self.base.stance_millis += delta;

    self.data.step(&mut self.base, delta);

    if let Some(ref mut collision) = self.collision {
      collision.update_isometry(&self.base);
    }
  }

  pub fn despawn(&mut self) -> bool {
    self.data.despawn_action(&mut self.base)
  }
}
