use opensrme_common::*;
use super::*;
use std::collections::HashMap;

#[derive(Hash, Debug, Copy, Clone, Eq, PartialEq)]
pub enum InputKey {
  Unknown = -1,

  Down = 0,
  Right = 1,
  Up = 2,
  Left = 3,

  Attack = 4,

  Context = 7, // LSB
  Exit = 8, // RSB

  Vehicle = 11 // Enter/Exit
}

pub fn platform_key_to_inputkey(key: Key) -> InputKey {
  if key.scancode == 41 {
    // esc
    return InputKey::Exit;
  }

  return match key.value as char {
    'a' => InputKey::Left,
    'd' => InputKey::Right,
    'w' => InputKey::Up,
    's' => InputKey::Down,
    'e' => InputKey::Context,
    'f' => InputKey::Attack,
    'v' => InputKey::Vehicle,
    _ => InputKey::Unknown
  }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct InputContext {
  pub keys: HashMap<InputKey, Time>,
  pub key_delta: HashMap<InputKey, bool>,
  pub buttons: HashMap<MouseButton, Time>,
  pub button_delta: HashMap<MouseButton, bool>,
  pub mouse: Vec3i,
  pub mouse_delta: Vec3i
}

impl InputContext {
  pub fn step(&mut self) {
    self.key_delta.clear();
    self.button_delta.clear();
    self.mouse_delta = Vec3i::default();
  }

  pub fn process_platform_event(&mut self, event: Event) {
    let context = globals::get_context();

    match event {
      Event::Key { key, pressed } => {
        let inputkey = platform_key_to_inputkey(key);

        if inputkey == InputKey::Unknown {
          return;
        }

        if pressed {
          self.keys.insert(inputkey, context.time);
        } else {
          self.keys.remove(&inputkey);
        }

        self.key_delta.insert(inputkey, pressed);
      },
      Event::MouseButton { pressed, button } => {
        if pressed {
          self.buttons.insert(button, context.time);
        } else {
          self.buttons.remove(&button);
        }

        self.button_delta.insert(button, pressed);
      },
      Event::MousePos { pos, delta } => {
        self.mouse = pos;
        self.mouse_delta = self.mouse_delta + delta;
      },
      _ => {}
    }
  }
}
