use super::*;

pub trait PlayerController {
  fn process(&mut self, delta: Time, data: &mut person::PlayerData);
}

pub struct ModernPlayerControls {}

fn get_movement_direction(context: &globals::Context) -> Option<Angle> {
  // TODO: sort by time

  let mut up = false;
  let mut down = false;
  if context.input.keys.contains_key(&input::InputKey::Up) {
    up = true;
    down = false;
  } else if context.input.keys.contains_key(&input::InputKey::Down) {
    up = false;
    down = true;
  }

  let mut left = false;
  let mut right = false;
  if context.input.keys.contains_key(&input::InputKey::Left) {
    left = true;
    right = false;
  } else if context.input.keys.contains_key(&input::InputKey::Right) {
    left = false;
    right = true;
  }

  if up {
    if left {
      Some(util::angles::RANGLE_NW)
    } else if right {
      Some(util::angles::RANGLE_NE)
    } else {
      Some(util::angles::RANGLE_N)
    }
  } else if down {
    if left {
      Some(util::angles::RANGLE_SW)
    } else if right {
      Some(util::angles::RANGLE_SE)
    } else {
      Some(util::angles::RANGLE_S)
    }
  } else if left {
    Some(util::angles::RANGLE_W)
  } else if right {
    Some(util::angles::RANGLE_E)
  } else {
    None
  }
}

impl ModernPlayerControls {
  pub fn new() -> Self {
    ModernPlayerControls {}
  }

  fn get_angle_from_mouse(player: &entity::Entity, context: &globals::Context) -> Option<Angle> {
    //let middle_delta = context.input.mouse - (context.platform.get_size() / 2);
    let game = globals::get_game();

    //let mouse_ingame_pos = context.input.mouse - game.main_camera_pos;
    let mouse_ingame_pos = game.screen_pos_to_game_pos(context.input.mouse);
    let middle_delta = mouse_ingame_pos - player.base.pos;

    if middle_delta.len2() < player.base.get_class().width / 2. {
      None
    } else {
      Some(util::vec_angle(middle_delta / 2.))
    }
  }
}

impl PlayerController for ModernPlayerControls {
  fn process(&mut self, delta: Time, data: &mut person::PlayerData) {
    let context = globals::get_context();
    let player = &mut globals::get_game().entities[0];

    let angle = ModernPlayerControls::get_angle_from_mouse(player, context);
    let has_angle = angle.is_some();
    if let Some(angle) = angle {
      player.base.angle = angle;
    }

      // TODO implement sliding

    if (player.base.stance == entity::EntityStance::Standing ||
        player.base.stance == entity::EntityStance::Aiming ||
        player.base.stance == entity::EntityStance::Walking ||
        player.base.stance == entity::EntityStance::Running) {
      let mut movement_direction = None;
      if has_angle {
        movement_direction = get_movement_direction(context);
      }

      if let Some(movement_direction) = movement_direction {
        player.base.speed = person::PLAYER_SPEED;
        player.base.set_new_stance(entity::EntityStance::Running);

        player.base.strafe(movement_direction, delta);
      } else {
        player.base.speed = 0.;

        if player.base.stance.is_self_moving() {
          player.base.set_new_stance(entity::EntityStance::Standing);
        }
      }
    }
  }
}
