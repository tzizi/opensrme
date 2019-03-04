use super::*;
use entity::*;

pub const PLAYER_SPEED: FScalar = 60.;

#[derive(Debug, Clone, PartialEq)]
pub struct SidewalkData {
  pub walking_direction: Vec3f,
  pub walking_angle: Angle
}

pub struct PlayerData {
}

pub enum PersonData {
  Base,
  Sidewalk(SidewalkData),
  Player(PlayerData)
}

impl PersonData {
  pub fn new(entity_type: EntityType) -> Self {
    if (entity_type == EntityType::Pedestrian ||
        entity_type == EntityType::VehiclePedestrian ||
        // TODO
        entity_type == EntityType::Gangster ||
        // TODO
        entity_type == EntityType::Type7 ||
        // TODO
        entity_type == EntityType::Police) {
      PersonData::Sidewalk(SidewalkData::new())
    } else if entity_type == EntityType::Player {
      PersonData::Player(PlayerData::new())
    } else {
      PersonData::Base
    }
  }
}

fn step_base_person(entity: &mut EntityBase, delta: Time) -> bool {
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

  entity.step_route(delta);
  return false;
}

impl SidewalkData {
  pub fn new() -> Self {
    SidewalkData {
      walking_direction: Vec3f::default(),
      walking_angle: 0.
    }
  }

  fn pick_sidewalk_direction(&mut self, entity: &mut EntityBase) {
    let angle: Angle = util::pick_int(4) as f64 * util::HALF_PI;
    let class = entity.get_class().clone();
    self.walking_direction.x = class.width * angle.cos();
    self.walking_direction.y = class.height * angle.sin();
    self.walking_angle = angle;
  }

  fn step(&mut self, entity: &mut EntityBase, delta: Time) -> bool {
    if step_base_person(entity, delta) {
      return true;
    }

    match entity.stance {
      EntityStance::Standing => {
        let level = &globals::get_game().level;

        for _i in 0..4 {
          self.pick_sidewalk_direction(entity);
          if level::pos_is_sidewalk(level, entity.pos + self.walking_direction) {
            entity.set_new_stance(EntityStance::Walking);
            entity.speed = 15. + util::pick_float(15.);
            break;
          }
        }
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

impl PlayerData {
  pub fn new() -> Self {
    PlayerData {}
  }

  fn step(&mut self, entity: &mut EntityBase, delta: Time) {
    // TODO
    // if entity.flags & 0x4000 != 0 { return; }
    if step_base_person(entity, delta) {
      return;
    }

    // TODO: shooting, sliding

    let game = globals::get_game();
    game.playercontroller.process(delta, self);

    // TODO: check for nearby car (or possibly other objects?)
  }
}

fn get_palette_id(entity_type: EntityType) -> PaletteId {
  if entity_type == EntityType::Player {
    13
  } else if entity_type == EntityType::Police {
    0
  } else if (entity_type == EntityType::Type1 ||
             entity_type == EntityType::Pedestrian ||
             entity_type == EntityType::VehiclePedestrian ||
             entity_type == EntityType::Type7) {
    EntityBase::pick_npc_person_palette()
  } else if entity_type == EntityType::Gangster {
    // TODO
    0
  } else {
    // shouldn't happen
    0
  }
}

pub fn draw(entity: &EntityBase) {
  if entity.stance == EntityStance::Riding {
    return;
  }

  let context = globals::get_context();
  let class = entity.get_class();

  /*if class.clip == -1 {
      return;
    }*/

  let mut stance = entity.stance;
  if stance == EntityStance::Unknown {
    stance = EntityStance::Running;
  }

  //println!("{:?} {:?}", class, stance);

  let index = class.clip + stance as i32;
  if index == -1 {
    return;
  }

  let clip = &context.data.clips[index as usize];
  let clip_angle = &clip[util::get_angle_in_clip(entity.angle, clip.len())];
  let current_sprite = clip_angle[util::get_frame_in_clip(entity.stance_millis, 700, clip_angle.len())];

  let imageid = sprite::get_image_from_sprite(current_sprite).unwrap();
  sprite::draw_sprite_palette(current_sprite, entity.pos.into(), 0, &vec![(imageid, entity.palette)]);
}

impl EntityData for PersonData {
  fn init(&mut self, entity: &mut EntityBase) {
    entity.palette = get_palette_id(entity.entity_type);

    match entity.entity_type {
      EntityType::Player => {
        entity.hidden = false;
      },
      _ => {
      }
    }
  }

  fn get_collision_info(&self, entity: &EntityBase) -> Option<collision::ShapeInfo> {
    if entity.entity_type == EntityType::Player {
      let class = &entity.get_class();

      Some(collision::ShapeInfo {
        shape: collision::Shape::Circle(class.width as i32),
        weight: class.weight
      })
    } else {
      // FIXME: non-players are set to a circle, but without collision data.
      // this is only used for sliding.
      None
    }
  }

  fn step(&mut self, entity: &mut EntityBase, delta: Time) {
    match self {
      PersonData::Base => {
        step_base_person(entity, delta);
      },
      PersonData::Sidewalk(sidewalk) => {
        sidewalk.step(entity, delta);
      },
      PersonData::Player(player) => {
        player.step(entity, delta);
      }
    }
  }

  fn draw(&self, entity: &EntityBase) {
    draw(entity);
  }

  fn despawn_action(&mut self, entity: &mut EntityBase) -> bool {
    if entity.entity_type == EntityType::VehiclePedestrian {
      entity.hidden = true;
      false
    } else {
      true
    }
  }

  fn spawn(&mut self, _entity: &mut EntityBase, pos: Vec3f) -> Option<Vec3f> {
    let game = globals::get_game();
    if level::pos_is_sidewalk(&game.level, pos) {
      Some(pos)
    } else {
      None
    }
  }
}
