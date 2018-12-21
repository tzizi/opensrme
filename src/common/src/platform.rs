use std::io::*;
use super::archive::*;
use super::types::*;
pub type PlatformId = usize;

#[derive(Debug, Copy, Clone, PartialEq)]
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

  pub fn free_id(&mut self, _id: PlatformId) {}
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Key {
  pub value: u8,
  pub scancode: i32
}

#[derive(Hash, Debug, Copy, Clone, Eq, PartialEq)]
pub enum MouseButton {
  Unknown,
  Left,
  Right,
  Middle
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MouseScroll {
  Up,
  Down
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Event {
  Quit,
  Resize(Vec3i),
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
    pos: Vec3i,
    delta: Vec3i
  }
}

pub const FLIP_H: u8 = 1;
pub const FLIP_V: u8 = 2;
pub type Flip = u8;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Rotate {
  pub angle: FScalar, // 0 = no rotation, 1 = 360
  pub origin: Vec3i
}

pub trait Platform {
  fn new(title: &str, width: i16, height: i16) -> Self where Self: Sized;
  fn close_window(&mut self);
  //fn wait_event(&mut self, window: PlatformId);
  fn poll_event(&mut self) -> Option<Event>;
  fn set_title(&mut self, title: &str);
  fn get_size(&self) -> Vec3i;

  fn new_image(&mut self, image: Image) -> PlatformId;
  fn get_image_size(&mut self, image_id: PlatformId) -> Option<Vec3i>;
  fn load_image(&mut self, image: &[u8]) -> PlatformId {
    if let Ok(image) = image::load_from_memory(image) {
      let image = image.to_rgba();

      self.new_image(Image {
        size: Vec3i::new2(image.width() as i32, image.height() as i32),
        data: image.into_raw()
      })
    } else {
      0
    }
  }
  fn load_image_from_filename(&mut self, archive: &Archive, image: &str) -> PlatformId {
    let mut bytes = vec![];

    if let Ok(mut file) = archive.open_file(image) {
      if let Ok(_) = file.read_to_end(&mut bytes) {
        self.load_image(&bytes[..])
      } else {
        0
      }
    } else {
      0
    }
  }
  fn unload_image(&mut self, image: PlatformId);

  fn reset(&mut self);
  fn translate(&mut self, pos: Vec3i);
  fn get_translation(&mut self) -> Vec3i;
  //fn scale(&mut self, scale: Vec3i);

  fn set_color(&mut self, color: Color);
  fn clear(&mut self);
  fn draw_region(&mut self, image: PlatformId,
                 x_src: IScalar, y_src: IScalar,
                 width: IScalar, height: IScalar,
                 flip: Flip,
                 rotate: Option<Rotate>,
                 x_dest: IScalar, y_dest: IScalar);
  fn fill_rect(&mut self, x: IScalar, y: IScalar, width: IScalar, height: IScalar);
  fn fill_rect_vec(&mut self, pos: Vec3i, size: Vec3i) {
    self.fill_rect(pos.x, pos.y, size.x, size.y);
  }

  fn swap(&mut self);
}
