use std::io::*;
use super::types::*;
pub type PlatformId = usize;

pub struct IdCache {
  current_id: PlatformId
}

impl IdCache {
  pub fn new() -> Self {
    IdCache {
      current_id: 1
    }
  }

  pub fn get_id(&mut self) -> PlatformId {
    let id = self.current_id;
    self.current_id += 1;

    id
  }

  pub fn free_id(&mut self, id: PlatformId) {
  }
}

pub struct Key {
  pub value: u8,
  pub scancode: i32
}

pub enum MouseButton {
  Unknown,
  Left,
  Right,
  Middle
}

pub enum MouseScroll {
  Up,
  Down
}

pub enum Event {
  Quit,
  Key {
    pressed: bool,
    key: Key
  },
  MouseButton {
    pressed: bool,
    button: MouseButton
  },
  MouseScroll(MouseScroll),
  MousePos {
    pos: Vec2i
  }
}

pub enum Flip {
  Horizontal,
  Vertical,
  Both
}

pub struct Rotate {
  pub angle: FScalar, // 0 = no rotation, 1 = 360
  pub origin: Vec2i
}

pub trait Platform {
  fn new(title: &str, width: i16, height: i16) -> Self;
  fn close_window(&mut self);
  //fn wait_event(&mut self, window: PlatformId);
  fn poll_event(&mut self) -> Option<Event>;

  fn new_image(&mut self, image: Image) -> PlatformId;
  fn load_image(&mut self, image: &[u8]) -> PlatformId {
    if let Ok(image) = image::load_from_memory(image) {
      let image = image.to_rgba();

      self.new_image(Image {
        size: Vec2i::new(image.width() as i32, image.height() as i32),
        data: image.into_raw()
      })
    } else {
      0
    }
  }
  fn unload_image(&mut self, image: PlatformId);

  fn clear(&mut self, color: Color);
  fn draw_region(&mut self, image: &PlatformId,
                 x_src: IScalar, y_src: IScalar,
                 width: IScalar, height: IScalar,
                 flip: Option<Flip>,
                 rotate: Option<Rotate>,
                 x_dest: IScalar, y_dest: IScalar);

  fn swap(&mut self);
}
