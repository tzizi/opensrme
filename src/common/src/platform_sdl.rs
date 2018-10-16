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

  textures: HashMap<PlatformId, sdl2::render::Texture>
}


// https://wiki.libsdl.org/SDLKeycodeLookup
// var nums = []; t.querySelectorAll("tr").forEach((el) => {var text = el.querySelector("td").innerText;var n = parseInt(text); if (!isNaN(n) && n < 256) {nums.push(n)}});
static kv_table: &'static [u8] = &[
  0, 8, 9, 13, 27, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 91, 92, 93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 127
];

fn key_to_ascii(key: sdl2::keyboard::Keycode) -> u8 {
  let code = key as u32;

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
    _                                         => return None
  })
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
      textures: HashMap::new()
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

  fn new_image(&mut self, image: Image) -> PlatformId {
    let texture = self.texture_creator.create_texture_static(Some(sdl2::pixels::PixelFormatEnum::ABGR8888),
                                                             image.size.x as u32,
                                                             image.size.y as u32);

    if let Ok(mut texture) = texture {
      texture.set_blend_mode(sdl2::render::BlendMode::Blend);
      texture.update(None, &image.data[..], image.size.x as usize * 4).unwrap();

      let id = self.cache.get_id();
      self.textures.insert(id, texture);
      id
    } else {
      0
    }
  }

  fn unload_image(&mut self, image: PlatformId) {
    if let Some(_) = self.textures.remove(&image) {
      self.cache.free_id(image);
    }
  }

  fn clear(&mut self, color: Color) {
    self.sdl_canvas.set_draw_color(sdl2::pixels::Color::RGBA(color.r, color.g, color.b, color.a));
    self.sdl_canvas.clear();
  }

  fn draw_region(&mut self, image: &PlatformId,
                 x_src: IScalar, y_src: IScalar,
                 width: IScalar, height: IScalar,
                 flip: Option<Flip>,
                 rotate: Option<Rotate>,
                 x_dest: IScalar, y_dest: IScalar) {
    if let Some(image) = self.textures.get(image) {
      let mut angle = 0.0;
      let mut center = None;
      if let Some(rotate) = rotate {
        angle = rotate.angle * 360.0;
        center = Some(sdl2::rect::Point::new(rotate.origin.x, rotate.origin.y));
      }

      let mut flip_horizontal = false;
      let mut flip_vertical = false;

      if let Some(flip) = flip {
        match flip {
          Flip::Horizontal => flip_horizontal = true,
          Flip::Vertical => flip_vertical = true,
          Flip::Both => {
            flip_horizontal = true;
            flip_vertical = true;
          }
        }
      }

      let mut width: u32 = width as u32;
      let mut height: u32 = height as u32;
      let query = image.query();
      if width > query.width {
        width = query.width;
      }

      if height > query.height {
        height = query.height;
      }

      let src = sdl2::rect::Rect::new(x_src, y_src, width, height);
      let dst = sdl2::rect::Rect::new(x_dest, y_dest, width, height);
      self.sdl_canvas.copy_ex(image, src, dst, angle, center, flip_horizontal, flip_vertical);
    }
  }

  fn swap(&mut self) {
    self.sdl_canvas.present();
  }
}
