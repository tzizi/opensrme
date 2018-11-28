use super::*;

pub trait Screen {
  fn init(&mut self) {}
  fn step(&mut self, delta: Time) {}
  fn draw(&mut self) {}
}

#[derive(Debug, Clone, PartialEq)]
pub struct GameScreen {
  pub level: Level,
  pub entities: Vec<entity::Entity>,
  pub main_camera_pos: Vec3i,
  pub camera: Camera,

  entity_spawn_counter: usize
}

impl GameScreen {
  pub fn new(levelid: LevelId) -> Self {
    let context = globals::get_context();

    let mut game = GameScreen {
      level: level::get_level_from_levelid(levelid),
      entities: vec![],
      main_camera_pos: Vec3i::default(),
      camera: Camera::default(),

      entity_spawn_counter: 0
    };

    game.camera.size = Vec3i::new2(240, 320);

    game
  }

  fn process_input(&mut self) {
    let context = globals::get_context();

    for key in context.input.key_delta.iter() {
      if !key.1 {
        continue;
      }

      match key.0 {
        input::InputKey::Exit => {
          context.running = false;
          break;
        },
        _ => {}
      }
    }

    if context.input.buttons.get(&MouseButton::Left).is_some() {
      self.main_camera_pos = self.main_camera_pos + context.input.mouse_delta;
    }

    if context.input.buttons.get(&MouseButton::Right).is_some() {
      self.camera.pos = context.input.mouse - self.main_camera_pos - (self.camera.size / 2);
    }
  }

  fn get_spawn_xy(current: IScalar, max: IScalar) -> Option<Vec3i> {
    if current >= max * 8 {
      return None;
    }

    const values: [[IScalar; 2]; 4] = [[0, -1], [1, 0], [0, 1], [-1, 0]];

    let id: usize = (current % 4) as usize;

    let mut x = max * values[id][0];
    let mut y = max * values[id][1];

    let temp = current / 4 + 1;
    let mut mult = temp / 2;

    if temp % 2 != 0 {
      mult *= -1;
    }

    x += mult * values[id][1];
    y += mult * values[id][0];

    Some(Vec3::new2(x, y))
  }

  fn can_spawn_at(level: &Level, entity_type: entity::EntityType, pos: Vec3f) -> bool {
    if entity_type.is_person() {
      return level::pos_is_sidewalk(level, pos);
    } else {
      return false;
    }
  }

  fn find_entity_spawn_point(level: &Level, entity_type: entity::EntityType,
                             x: IScalar, y: IScalar, border: IScalar) -> Option<Vec3f> {
    // TODO: do proper checks
    let x = x / level.tilesize.x as IScalar;
    let y = y / level.tilesize.x as IScalar;
    let border = border / level.tilesize.x as IScalar;

    let max = border * 8;
    let mut current = util::pick_int(max);
    loop {
      let spawn_xy = GameScreen::get_spawn_xy(current, border);
      if let Some(spawn_xy) = spawn_xy {
        let spawn_xy = spawn_xy + Vec3i::new2(x, y);

        if spawn_xy.x >= 0 && spawn_xy.y >= 0 &&
          spawn_xy.x < level.tiledata_size.x &&
          spawn_xy.y < level.tiledata_size.y {
            let pos = Vec3f::from(spawn_xy) * level.tilesize + level.tilesize / 2.;

            if GameScreen::can_spawn_at(level, entity_type, pos) {
              return Some(pos);
            }
          }

        current += 1;
      } else {
        break;
      }
    }

    None
  }

  fn apply_entity_spawn_point(level: &Level, entity: &mut entity::Entity, pos: Vec3i, border: IScalar) -> bool {
    let pos = GameScreen::find_entity_spawn_point(level, entity.entity_type, pos.x, pos.y, border);
    if let Some(pos) = pos {
      entity.pos = pos;
      true
    } else {
      false
    }
  }

  fn step_entity_spawn(&mut self) {
    loop {
      let mut entity_found = true;
      let entities_len = self.entities.len();
      let entity = &mut self.entities[self.entity_spawn_counter % entities_len];
      // todo: check for flag 0x10000 == 0
      match entity.entity_type {
        entity::EntityType::Player => {},
        entity::EntityType::Pedestrian => {
          if (entity.pos.x as IScalar - self.camera.middle().x).abs() > 320 ||
            (entity.pos.y as IScalar - self.camera.middle().y).abs() > 320 {
              GameScreen::apply_entity_spawn_point(&self.level, entity, self.camera.middle(), 240);
            }
        },
        entity::EntityType::Gangster => {
          if (entity.pos.x as IScalar - self.camera.middle().x).abs() > 320 ||
            (entity.pos.y as IScalar - self.camera.middle().y).abs() > 320 {
              GameScreen::apply_entity_spawn_point(&self.level, entity, self.camera.middle(), 240);
            }
        },
        _ => {
          entity_found = false;
        }
      }

      self.entity_spawn_counter += 1;
      if entity_found {
        break;
      }
    }
  }
}

impl Screen for GameScreen {
  fn init(&mut self) {
    self.entities = level::load_entities(&self.level);
  }

  fn step(&mut self, delta: Time) {
    self.process_input();
    self.step_entity_spawn();

    for entity in self.entities.iter_mut() {
      entity.step(delta);
    }
  }

  fn draw(&mut self) {
    let context = globals::get_context();

    context.platform.set_color(Color { r: 0, g: 0, b: 0, a: 255 });
    context.platform.clear();
    context.platform.reset();

    context.platform.translate(self.main_camera_pos);
    //context.platform.draw_region(&image, 0, 0, x, x, 0, None, 50, 10);

    level::draw_level_layer(&self.level.layer1);
    level::draw_shadows(&self.level);
    level::draw_objects(&self.level);
    level::draw_level_layer(&self.level.layer2);

    for entity in self.entities.iter() {
      entity.draw();
    }

    // draw camera
    context.platform.set_color(Color { r: 255, g: 0, b: 0, a: 255 });
    context.platform.fill_rect(self.camera.pos.x - 2,
                               self.camera.pos.y - 2,
                               self.camera.size.x + 2,
                               4);
    context.platform.fill_rect(self.camera.pos.x - 2,
                               self.camera.pos.y + self.camera.size.y - 2,
                               self.camera.size.x + 2,
                               4);
    context.platform.fill_rect(self.camera.pos.x - 2,
                               self.camera.pos.y - 2,
                               4,
                               self.camera.size.y + 2);
    context.platform.fill_rect(self.camera.pos.x + self.camera.size.x - 2,
                               self.camera.pos.y - 2,
                               4,
                               self.camera.size.y + 2);
  }
}
