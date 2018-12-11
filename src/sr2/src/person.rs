use super::*;
use entity::*;

#[derive(Debug, Clone, PartialEq)]
pub struct PersonData {
  pub walking_direction: Vec3f,
  pub walking_angle: Angle
}

impl PersonData {
  pub fn new() -> Self {
    PersonData {
      walking_direction: Vec3f::default(),
      walking_angle: 0.
    }
  }

  fn step_person(&mut self, entity: &mut EntityBase) -> bool {
    if entity.stance == EntityStance::Dead {
      return true;
    }

    entity.update_prev();

    match entity.stance {
      EntityStance::Punching => {
        // TODO
      },
      EntityStance::Shooting => {
        // TODO
      },
      EntityStance::Aiming => {
        if entity.stance_millis > 1000 {
          entity.set_new_stance(EntityStance::Standing)
        }
      },
      EntityStance::LyingDown => {
        if entity.stance_millis > 3000 {
          entity.set_new_stance(EntityStance::Standing)
        }

        return true;
      },
      _ => {}
    }

    // TODO: step route
    return false;
  }

  fn pick_sidewalk_direction(&mut self, entity: &mut EntityBase) {
    let angle: Angle = util::pick_int(4) as f64 * util::HALF_PI;
    let class = entity.get_class().clone();
    self.walking_direction.x = class.width * angle.cos();
    self.walking_direction.y = class.height * angle.sin();
    self.walking_angle = angle;
  }

  fn move_forward(entity: &mut EntityBase, delta: Time, speed: FScalar) {
    if speed == 0. {
      return;
    }

    let amount = (delta as FScalar) / 1000. * speed;
    entity.pos.x += amount * entity.angle.cos();
    entity.pos.y += amount * entity.angle.sin();
  }

  fn step_sidewalk_path(&mut self, entity: &mut EntityBase, delta: Time) -> bool {
    if self.step_person(entity) {
      return true;
    }

    match entity.stance {
      EntityStance::Standing => {
        entity.speed = 15. + util::pick_float(15.);
        self.pick_sidewalk_direction(entity);
        entity.set_new_stance(EntityStance::Walking);
      },
      EntityStance::Walking => {
        let old_angle = entity.angle;

        entity.angle = self.walking_angle;

        entity.move_forward(delta);

        if !level::pos_is_sidewalk(&globals::get_game().level, entity.pos + self.walking_direction) {
          entity.pos.x = entity.prev_pos.x;
          entity.pos.y = entity.prev_pos.y;
          entity.angle = old_angle;
          entity.set_new_stance(EntityStance::Standing);
        }
      },
      _ => {}
    }

    return false;
  }
}

impl EntityData for PersonData {
  fn step(&mut self, entity: &mut EntityBase, delta: Time) {
    match entity.entity_type {
      EntityType::Type1 => {
        self.step_person(entity);
      },
      EntityType::Pedestrian => {
        self.step_sidewalk_path(entity, delta);
      },
      EntityType::VehiclePedestrian => {
        self.step_sidewalk_path(entity, delta);
      },
      EntityType::Gangster => {
        // TODO
        self.step_sidewalk_path(entity, delta);
      },
      _ => {}
    }
  }

  fn draw(&self, entity: &EntityBase) {
    if entity.stance == EntityStance::Riding {
      return;
    }

    let context = globals::get_context();
    let class = entity.get_class();

    if class.clip == -1 {
      return;
    }

    let mut stance = entity.stance;
    if stance == EntityStance::Unknown {
      stance = EntityStance::Running;
    }

    //println!("{:?} {:?}", class, stance);

    let clip = &context.data.clips[(class.clip + stance as i32) as usize];
    // TODO: get the correct index from the angle
    let clip_angle = &clip[util::get_angle_in_clip(entity.angle, clip.len())];
    // TODO: animate properly
    let current_sprite = clip_angle[util::get_frame_in_clip(entity.stance_millis, 700, clip_angle.len())];

    // TODO: palettes
    sprite::draw_sprite(current_sprite, entity.pos.into(), 0);
  }
}
