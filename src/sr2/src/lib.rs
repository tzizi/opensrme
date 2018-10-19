extern crate opensrme_common;
extern crate encoding;

mod types;
use types::*;
mod sprite;
mod bin_all;
use bin_all::*;
mod level;
use level::*;

#[macro_use]
use opensrme_common::*;

use std::io;
use std::io::Read;

pub fn check(archive: &Archive) -> io::Result<bool> {
  let manifest = archive.get_manifest()?;

  Ok(check_manifest!(manifest,
                     "MIDlet-Name": "Saints Row 2"))
}

pub fn main(archive: &Archive, args: Vec<String>) {
  let mut platform = SDL2Platform::new("Saints Row 2", 800, 800);

  let splash = platform.load_image_from_filename(archive, "Title.png");
  platform.set_color(Color { r: 0, g: 0, b: 0, a: 255 });
  platform.clear();
  platform.draw_region(&splash, 0, 0, 240, 300, 0, None, 0, 10);
  platform.swap();

  let datacontext = read_bin_all(archive).unwrap();

  let mut images = vec![];
  for i in datacontext.images.iter() {
    println!("{}", i);
    images.push(PaletteImage {
      filename: i.clone(),
      image: platform.load_image_from_filename(archive, &i[..])
    });
  }

  let level = read_level(&mut archive.open_file("Street.lvl").unwrap()).unwrap();
  println!("{:?}", level);

  let mut context = Context {
    platform: Box::new(platform),
    time: instant_get_millis(),
    delta: 0,
    data: datacontext,
    images,
    levels: vec![]
  };

  let image = context.platform.load_image_from_filename(archive, "Car_Police.png");

  let mut running = true;
  let mut x = 0;
  let mut current_sprite: SpriteId = 1359;
  let mut leftpressed = false;
  let mut offset = Vec2i::new(0, 0);

  let mut last_second = context.time;
  let mut fps = 0;
  while running {
    let lasttime = context.time;
    context.time = instant_get_millis();
    context.delta = context.time - lasttime;

    while let Some(event) = context.platform.poll_event() {
      match event {
        Event::Quit => {
          running = false;
          break
        },
        Event::Key { key, pressed } => {
          if !pressed {
            if key.scancode == 41 {
              // esc
              running = false;
              break;
            }

            if key.value == 'a' as u8 {
              current_sprite -= 1;
              println!("{} {:?}", current_sprite, context.data.sprites[current_sprite as usize]);
            } else if key.value == 'd' as u8 {
              current_sprite += 1;
              println!("{} {:?}", current_sprite, context.data.sprites[current_sprite as usize]);
            }
          }
        },
        Event::MouseButton { pressed, button } => {
          if button == MouseButton::Left {
            leftpressed = pressed;
          }
        },
        Event::MousePos { delta, .. } => {
          if leftpressed {
            offset = offset + delta;
          }
        },
        _ => {}
      }
    }

    x += 5;
    x = x % 500;

    context.platform.set_color(Color { r: 0, g: 0, b: 0, a: 255 });
    context.platform.clear();
    context.platform.reset();

    context.platform.translate(offset);
    //context.platform.draw_region(&image, 0, 0, x, x, 0, None, 50, 10);

    draw_level_layer(&mut context, &level.layer1);
    draw_shadows(&mut context, &level);
    draw_level_layer(&mut context, &level.layer2);

    sprite::draw_sprite(&mut context, current_sprite, Vec2i::new(50, 50), 0);

    context.platform.swap();

    if context.time - last_second >= 1000 {
      last_second = context.time;
      //println!("{}", fps);
      fps = 0;
    } else {
      fps += 1;
    }

    let mut sleep = 16;
    let millis = instant_get_millis() - context.time;
    if millis < sleep {
      sleep -= millis;
      std::thread::sleep(std::time::Duration::from_millis(sleep));
    }
  }

}

pub static GAME: Game = Game {
  name: "Saints Row 2",

  main: main,
  check: check
};
