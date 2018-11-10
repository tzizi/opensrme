use super::*;


pub enum EntityType {
  // 1
  Type1,
  // 2
  Player,
  // 3
  Pedestrian,
  // 4
  VehiclePedestrian,
  // 5
  Gangster,
  // 6
  Police,
  // 7
  Type7,
  // 8, vehicle
  Type8,
  // 9
  DrivenVehicle,
  // 10
  MovingVehicle,
  // 11
  EnemyVehicle,
  // 12
  PoliceCar,
  // 13
  Pickup,
  // 14
  Type14,
  // 15
  Type15,
  // 16
  GarageDoor,
  // 17
  Type17,
  // 18
  Type18,
  // 19
  Type19,
  // 20 (sports car?)
  Type20
}

pub struct Entity {
  pub classid: ClassId,
  pub entity_type: i32,
  pub pos: Vec3f,
  pub angle: Angle
}

impl Entity {
  pub fn new(classid: ClassId) -> Self {
    let context = get_context();

    Entity {
      classid,
      entity_type: context.data.classes[classid as usize].entity_type,
      pos: Vec3f::new2(0., 0.),
      angle: 0.
    }
  }

  /*pub fn step(&mut self, delta: Time) {
    match self.entity_type {
      10 => {

      }
    }
  }*/
}
