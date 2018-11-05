use super::*;

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
