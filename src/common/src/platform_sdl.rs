use super::types::*;
use super::platform::*;
use std::io::*;
use std::collections::*;

use sdl2::event::Event as SEvent;

pub struct SDL2Platform {
  cache: IdCache,

  sdl_context: sdl2::Sdl,
  sdl_video: sdl2::VideoSubsystem,
  sdl_events: sdl2::EventPump,
  sdl_canvas: sdl2::render::Canvas<sdl2::video::Window>,
  texture_creator: sdl2::render::TextureCreator<sdl2::video::WindowContext>,

  textures: HashMap<PlatformId, sdl2::render::Texture>,
  image_sizes: HashMap<PlatformId, Vec3i>,

  offset: Vec3i,
  scale: FScalar
}


// https://wiki.libsdl.org/SDLKeycodeLookup
// var nums = []; t.querySelectorAll("tr").forEach((el) => {var text = el.querySelector("td").innerText;var n = parseInt(text); if (!isNaN(n) && n < 256) {nums.push(n)}});
static kv_table: &'static [u8] = &[
  0, 8, 9, 13, 27, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 91, 92, 93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 127
];

fn key_to_ascii(key: sdl2::keyboard::Keycode) -> u8 {
  let code = key as u32;

  if code < 256 {
    return code as u8;
  } else {
    return 0;
  }

  println!("{} {}", code, kv_table.len());

  if (code as usize) < kv_table.len() {
    kv_table[code as usize]
  } else {
    0
  }
}

fn key_event(pressed: bool,
             keycode: Option<sdl2::keyboard::Keycode>,
             scancode: Option<sdl2::keyboard::Scancode>) -> Option<Event> {
  if let Some(keycode) = keycode {
    if let Some(scancode) = scancode {
      return Some(Event::Key {
        pressed,
        key: Key {
          value: key_to_ascii(keycode),
          scancode: scancode as i32
        }
      })
    }
  }

  None
}

fn mouseb_event(pressed: bool, mouse_btn: sdl2::mouse::MouseButton) -> Event {
  Event::MouseButton {
    pressed,
    button: match mouse_btn {
      sdl2::mouse::MouseButton::Left => MouseButton::Left,
      sdl2::mouse::MouseButton::Right => MouseButton::Right,
      sdl2::mouse::MouseButton::Middle => MouseButton::Middle,
      _ => MouseButton::Unknown
    }
  }
}

/*fn get_key(button: ButtonArgs, key: PKey) -> Key {
  Key {
    value: key_to_ascii(key),
    scancode: button.scancode.unwrap()
  }
}*/

fn get_event(event: SEvent) -> Option<Event> {
  Some(match event {
    SEvent::Quit {..}                         => Event::Quit,
    SEvent::KeyDown { keycode, scancode, .. } => return key_event(true, keycode, scancode),
    SEvent::KeyUp { keycode, scancode, .. }   => return key_event(false, keycode, scancode),
    SEvent::MouseButtonDown { mouse_btn, .. } => mouseb_event(true, mouse_btn),
    SEvent::MouseButtonUp { mouse_btn, .. }   => mouseb_event(false, mouse_btn),
    SEvent::MouseMotion { x, y, xrel, yrel, .. } => {
      Event::MousePos {
        pos: Vec3i::new2(x, y),
        delta: Vec3i::new2(xrel, yrel)
      }
    },
    SEvent::MouseWheel { y, .. } => {
      Event::MouseScroll(
        if y < 0 {
          MouseScroll::Down
        } else if y > 0 {
          MouseScroll::Up
        } else {
          return None
        }
      )
    },
    SEvent::Window { win_event, .. }             => {
      match win_event {
        sdl2::event::WindowEvent::SizeChanged(x, y) => {
          Event::Resize(Vec3i::new2(x, y))
        },
        _                                           => return None
      }
    },
    _                                         => return None
  })
}

fn create_rect(x: IScalar, y: IScalar,
               width: IScalar, height: IScalar) -> sdl2::rect::Rect {
  sdl2::rect::Rect::new(x, y, width as u32, height as u32)
}

fn iscale(scale: FScalar, x: IScalar) -> IScalar {
  (x as FScalar * scale).ceil() as IScalar
}

fn create_scaled_rect(
  scale: FScalar,
  x: IScalar, y: IScalar,
  width: IScalar, height: IScalar) -> sdl2::rect::Rect {
  create_rect(iscale(scale, x), iscale(scale, y),
              iscale(scale, width), iscale(scale, height))
}

// rewrite to use gfx/glium + gfx_graphics
// or rewrite so that event loop is on own thread
impl Platform for SDL2Platform {
  fn new(title: &str, width: i16, height: i16) -> Self {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let eventpump = sdl_context.event_pump().unwrap();

    let window = video_subsystem.window(
      title,
      width as u32, height as u32
    ).build().unwrap();

    let mut canvas = window.into_canvas().accelerated().build().unwrap();
    canvas.set_blend_mode(sdl2::render::BlendMode::Blend);
    let texture_creator = canvas.texture_creator();

    SDL2Platform {
      cache: IdCache::new(),
      sdl_context,
      sdl_video: video_subsystem,
      sdl_events: eventpump,
      sdl_canvas: canvas,
      texture_creator: texture_creator,
      textures: HashMap::new(),
      image_sizes: HashMap::new(),
      offset: Vec3i::new2(0, 0),
      scale: 1.
    }
  }

  fn close_window(&mut self) {
    self.sdl_canvas.window_mut().hide();
  }

  fn poll_event(&mut self) -> Option<Event> {
    if let Some(event) = self.sdl_events.poll_event() {
      get_event(event)
    } else {
      None
    }
  }

  fn set_title(&mut self, title: &str) {
    self.sdl_canvas.window_mut().set_title(title);
  }

  fn get_size(&self) -> Vec3i {
    let (x, y) = self.sdl_canvas.window().drawable_size();
    Vec3i::new2(x as IScalar, y as IScalar)
  }

  fn new_image(&mut self, image: Image) -> PlatformId {
    let texture = self.texture_creator.create_texture_static(Some(sdl2::pixels::PixelFormatEnum::ABGR8888),
                                                             image.size.x as u32,
                                                             image.size.y as u32);

    if let Ok(mut texture) = texture {
      texture.set_blend_mode(sdl2::render::BlendMode::Blend);
      texture.update(None, &image.data[..], image.size.x as usize * 4).unwrap();

      let id = self.cache.get_id();
      self.textures.insert(id, texture);
      self.image_sizes.insert(id, image.size);
      id
    } else {
      0
    }
  }

  fn get_image_size(&mut self, image_id: PlatformId) -> Option<Vec3i> {
    if let Some(size) = self.image_sizes.get(&image_id) {
      Some(*size)
    } else {
      None
    }
  }

  fn unload_image(&mut self, image: PlatformId) {
    if let Some(_) = self.textures.remove(&image) {
      self.cache.free_id(image);
    }
  }

  fn reset_translation(&mut self) {
    self.offset = Vec3i::default();
  }

  fn translate(&mut self, pos: Vec3i) {
    self.offset = self.offset + pos;
  }

  fn get_translation(&self) -> Vec3i {
    self.offset
  }

  fn reset_scale(&mut self) {
    self.scale = 1.;
  }

  fn scale(&mut self, scale: FScalar) {
    self.scale *= scale;
  }

  fn get_scale(&self) -> FScalar {
    self.scale
  }

  fn set_color(&mut self, color: Color) {
    self.sdl_canvas.set_draw_color(sdl2::pixels::Color::RGBA(color.r, color.g, color.b, color.a));
  }

  fn clear(&mut self) {
    self.sdl_canvas.clear();
  }

  fn draw_region(&mut self, image: PlatformId,
                 x_src: IScalar, y_src: IScalar,
                 width: IScalar, height: IScalar,
                 flip: Flip,
                 rotate: Option<Rotate>,
                 x_dest: IScalar, y_dest: IScalar) {
    if let Some(image) = self.textures.get(&image) {
      let mut angle = 0.0;
      let mut center = None;
      if let Some(rotate) = rotate {
        angle = rotate.angle * 360.0;
        center = Some(sdl2::rect::Point::new(rotate.origin.x, rotate.origin.y));
      }

      let mut width = width;
      let mut height = height;
      let query = image.query();
      if width > query.width as IScalar {
        width = query.width as IScalar ;
      }

      if height > query.height as IScalar {
        height = query.height as IScalar;
      }

      //let src = sdl2::rect::Rect::new(x_src, y_src, (width as i32 + x_src) as u32, (height as i32 + y_src) as u32);
      //let dst = sdl2::rect::Rect::new(x_dest, y_dest, (width as i32 + x_dest) as u32, (height as i32 + y_dest) as u32);
      let src = create_rect(x_src, y_src, width, height);
      let dst = create_scaled_rect(self.scale, x_dest + self.offset.x, y_dest + self.offset.y, width, height);
      self.sdl_canvas.copy_ex(image, src, dst, angle, center, (flip & FLIP_H) != 0, (flip & FLIP_V) != 0).unwrap();
    }
  }

  fn fill_rect(&mut self, x: IScalar, y: IScalar, width: IScalar, height: IScalar) {
    self.sdl_canvas.fill_rect(Some(create_scaled_rect(self.scale, x + self.offset.x, y + self.offset.y, width, height))).unwrap();
  }

  fn swap(&mut self) {
    self.sdl_canvas.present();
  }
}
