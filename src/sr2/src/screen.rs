use super::*;

pub trait Screen {
  fn init(&mut self) {}
  fn step(&mut self, _delta: Time) {}
  fn draw(&mut self) {}
}

//#[derive(Debug, Clone, PartialEq)]
pub struct GameScreen {
  pub level: Level,
  pub levelid: LevelId,
  pub entities: Vec<entity::Entity>,
  pub entity_ids: Vec<EntityId>,
  pub main_camera_pos: Vec3i,
  pub camera: Camera,
  pub scale: FScalar,

  pub vehicle_state: vehicle::VehicleState,
  pub entity_spawn_counter: usize
}

impl GameScreen {
  pub fn new(levelid: LevelId) -> Self {
    //let context = globals::get_context();

    let mut game = GameScreen {
      level: level::get_level_from_levelid(levelid),
      levelid,
      entities: vec![],
      entity_ids: vec![],
      main_camera_pos: Vec3i::default(),
      camera: Camera::default(),
      scale: 1.,

      vehicle_state: vehicle::VehicleState::new(),
      entity_spawn_counter: 0
    };

    game.camera.size = Vec3i::new2(240, 320);

    game
  }

  fn handle_collision(entity1: &mut entity::Entity, entity2: Option<&mut entity::Entity>, response: Vec3f) {
    //entity1.on_collision(entity2, response);
    let end_pos = entity1.base.pos - response;
    entity1.set_pos(end_pos);
  }

  fn get_entity_response(entity1: &entity::Entity, entity2: &entity::Entity) -> Option<Vec3f> {
    if let Some(ref collision1) = entity1.collision {
      if let Some(ref collision2) = entity2.collision {
        return collision1.get_response_vector(collision2);
      }
    }

    None
  }

  fn step_collision(&mut self, _delta: Time) {
    let entities_len = self.entities.len();

    for entity1_id in 0..entities_len {
      //let entity1 = &mut self.entities[entity1_id];

      if !self.entities[entity1_id].is_physical() {
        continue;
      }

      for entity2_id in entity1_id+1..entities_len {
        if !self.entities[entity2_id].is_physical() {
          continue;
        }

        if let Some(response) = GameScreen::get_entity_response(&self.entities[entity1_id],
                                                                &self.entities[entity2_id]) {
          let mut splitted = self.entities[..].split_at_mut(entity1_id + 1);
          let entity1 = &mut splitted.0[entity1_id];
          let entity2 = &mut splitted.1[entity2_id - entity1_id - 1];
          GameScreen::handle_collision(entity1, Some(entity2), response);
        }
      }
    }
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
      self.main_camera_pos = self.main_camera_pos + Vec3i::from(Vec3f::from(context.input.mouse_delta) / self.scale);
    }

    if context.input.buttons.get(&MouseButton::Right).is_some() {
      self.camera.pos = Vec3i::from(Vec3f::from(context.input.mouse) / self.scale) - self.main_camera_pos - (self.camera.size / 2);
    }

    if context.input.mouse_scroll != 0 {
      let oldpos = Vec3i::from(Vec3f::from(context.input.mouse) / self.scale);
      self.scale -= context.input.mouse_scroll as FScalar * 0.1;
      if self.scale < 0.1 {
        self.scale = 0.1;
      }
      let newpos = Vec3i::from(Vec3f::from(context.input.mouse) / self.scale);
      self.main_camera_pos = self.main_camera_pos + (newpos - oldpos);
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

  fn find_entity_spawn_point(level: &Level, entity: &mut entity::Entity,
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

            if let Some(pos) = entity.spawn(pos) {
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
    let pos = GameScreen::find_entity_spawn_point(level, entity, pos.x, pos.y, border);
    return pos.is_some();
    /*if let Some(pos) = pos {
      entity.base.pos = pos;
      entity.spawn();
      true
    } else {
      false
    }*/
  }

  fn step_entity_despawn(&mut self) {
    loop {
      let mut entity_found = true;

      let entities_len = self.entities.len(); // because of borrow
      let entity = &mut self.entities[self.entity_spawn_counter % entities_len];

      // TODO: check for flag 0x10000 == 0
      if entity.base.entity_type.is_npc() {
        if self.camera.out_of_screen(entity.base.pos.into()) {
          if entity.despawn() {
            GameScreen::apply_entity_spawn_point(&self.level, entity, self.camera.middle(), self.camera.size.min2());
          }
        }
      } else if entity.base.entity_type != entity::EntityType::Player {
        entity_found = false;
      }

      self.entity_spawn_counter += 1;
      if entity_found {
        break;
      }
    }
  }

  fn create_entity_ids(&mut self) {
    self.entity_ids = vec![];

    for i in 0..self.entities.len() {
      self.entity_ids.push(i as EntityId);
    }

    GameScreen::update_entity_ids(&mut self.entity_ids, &self.entities);
  }

  fn update_entity_ids(entity_ids: &mut Vec<EntityId>, entities: &Vec<entity::Entity>) {
    entity_ids.sort_by(|a, b| {
      let entity_a = &entities[*a as usize];
      let entity_b = &entities[*b as usize];

      entity_a.base.sort_order.cmp(&entity_b.base.sort_order)
    });
  }
}

impl Screen for GameScreen {
  fn init(&mut self) {
    level::load_images(self.levelid);
    image::load_image(6, 12);
    image::load_image(6, 13);
    image::load_image(8, 12);
    for i in 1..4 {
      image::load_image(18, i);
    }
    for i in 1..4 {
      image::load_image(5, i);
    }
    for i in 1..4 {
      image::load_image(7, i);
    }
    for i in 1..4 {
      image::load_image(13, i);
    }
    self.entities = level::load_entities(&self.level);
    self.create_entity_ids();
  }

  fn step(&mut self, delta: Time) {
    self.process_input();

    self.vehicle_state.step();

    self.step_entity_despawn();

    for entity in self.entities.iter_mut() {
      /*if (entity.base.route.route.is_none() &&
          entity.base.route.routeid.is_some()) {
        entity.base.route.set_route_to_routeid();
        entity.base.set_new_stance(entity::EntityStance::Running);
        entity.base.speed = 20.;
      }*/

      entity.step(delta);
    }

    self.step_collision(delta);
  }

  fn draw(&mut self) {
    let context = globals::get_context();

    context.platform.set_color(Color { r: 0, g: 0, b: 0, a: 255 });
    context.platform.clear();
    context.platform.reset();

    context.platform.translate(self.main_camera_pos);
    context.platform.scale(self.scale);
    //context.platform.draw_region(&image, 0, 0, x, x, 0, None, 50, 10);

    GameScreen::update_entity_ids(&mut self.entity_ids, &self.entities);

    level::draw_level_layer(&self.level.layer1);
    level::draw_shadows(&self.level);
    level::draw_objects(&self.level, &self.entities, &self.entity_ids);
    level::draw_level_layer(&self.level.layer2);



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
