extern crate opensrme_common;
extern crate encoding;
extern crate rand;

mod types;
use types::*;
mod globals;
use globals::*;
mod sprite;
mod bin_all;
use bin_all::*;
mod level;
use level::*;
mod entity;
mod route;
mod util;
mod input;

#[macro_use]
use opensrme_common::*;

use std::io;

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
  println!("{:?}", datacontext.levels);

  let mut images = vec![];
  for i in datacontext.images.iter() {
    println!("{}", i);
    images.push(PaletteImage {
      filename: i.clone(),
      image: platform.load_image_from_filename(archive, &i[..])
    });
  }

  //println!("{:?}", datacontext);

  /*for i in datacontext.effects.iter() {
    println!("{:?}", i);
  }*/

  let level = read_level(&mut archive.open_file("Street.lvl").unwrap()).unwrap();
  println!("{} {}", level.tilesizex, level.tilesizey);
  //println!("{:?}", level);

  if false {
    for y in 0..level.tiledata_size.y {
      for x in 0..level.tiledata_size.x {
        let tile = level.tiledata[(y * level.tiledata_size.x + x) as usize];
        print!("{:3}", tile);
      }
      println!("");
    }
  }

  let mut game = GameContext {
    entities: vec![],
    level: level,
    camera: Camera::default()
  };

  let mut context = Context {
    platform: Box::new(platform),
    time: instant_get_millis(),
    delta: 0,
    data: datacontext,
    images,
    levels: vec![],
    game,
    input: input::InputContext::default()
  };

  set_context(context);
  let mut context = get_context();

  context.game.camera.size = Vec3i::new2(240, 320);

  let level = &context.game.level;

  load_entities(&level);

  //let image = context.platform.load_image_from_filename(archive, "Car_Police.png");

  let mut running = true;
  let mut x = 0;
  let draw_clip = false;
  let mut current_sprite: usize = 1200;
  let mut current_orientation: usize = 0;
  let mut leftpressed = false;
  let mut offset = Vec3i::new2(0, 0);

  let mut last_second = context.time;
  let mut fps = 0;
  let mut last_key_time: Time = 0;
  let mut entity_spawn_counter: usize = 0;
  while running {
    let lasttime = context.time;
    context.time = instant_get_millis();
    context.delta = context.time - lasttime;

    context.input.step();

    while let Some(event) = context.platform.poll_event() {
      if let Event::Quit = event {
        running = false;
        break;
      }

      context.input.process_platform_event(event);
    }

    if !running {
      break;
    }

    for key in context.input.key_delta.iter() {
      if !key.1 {
        continue;
      }

      match key.0 {
        input::InputKey::Exit => {
          running = false;
          break;
        },
        input::InputKey::Left => {
          if current_sprite == 0 && draw_clip {
            current_sprite = context.data.clips.len() - 1;
          } else {
            current_sprite -= 1;
          }
          current_orientation = 0;
          if draw_clip {
            println!("{} {:?}", current_sprite, context.data.clips[current_sprite as usize]);
          } else {
            println!("{}", current_sprite);
          }
        },
        input::InputKey::Right => {
          current_sprite += 1;
          if current_sprite as usize >= context.data.clips.len() && draw_clip {
            current_sprite = 0;
          }
          current_orientation = 0;
          if draw_clip {
            println!("{} {:?}", current_sprite, context.data.clips[current_sprite as usize]);
          } else {
            println!("{}", current_sprite);
          }
        },
        input::InputKey::Up => {
          if current_orientation == 0 {
            current_orientation = context.data.clips[current_sprite].len() - 1;
          } else {
            current_orientation -= 1;
          }
        },
        input::InputKey::Down => {
          current_orientation += 1;
          if current_orientation >= context.data.clips[current_sprite].len() {
            current_orientation = 0;
          }
        },
        _ => {}
      }
    }

    if context.input.buttons.get(&MouseButton::Left).is_some() {
      offset = offset + context.input.mouse_delta;
    }

    if context.input.buttons.get(&MouseButton::Right).is_some() {
      context.game.camera.pos = context.input.mouse - offset - (context.game.camera.size / 2);
    }

    x += 5;
    x = x % 500;

    // entity spawn
    loop {
      let mut entity_found = true;
      let entities_len = context.game.entities.len();
      let entity = &mut context.game.entities[entity_spawn_counter % entities_len];
      // todo: check for flag 0x10000 == 0
      match entity.entity_type {
        entity::EntityType::Player => {
        },
        entity::EntityType::Pedestrian => {
          if (entity.pos.x as IScalar - context.game.camera.middle().x).abs() > 320 ||
            (entity.pos.y as IScalar - context.game.camera.middle().y).abs() > 320 {
              entity.pos = context.game.camera.middle().into();
            }
        },
        entity::EntityType::Gangster => {
          if (entity.pos.x as IScalar - context.game.camera.middle().x).abs() > 320 ||
            (entity.pos.y as IScalar - context.game.camera.middle().y).abs() > 320 {
              entity.pos = context.game.camera.middle().into();
            }
        },
        _ => {
          entity_found = false;
        }
      }

      entity_spawn_counter += 1;
      if entity_found {
        break;
      }
    }

    for entity in context.game.entities.iter_mut() {
      entity.step(context.delta);
    }

    context.platform.set_color(Color { r: 0, g: 0, b: 0, a: 255 });
    context.platform.clear();
    context.platform.reset();

    context.platform.translate(offset);
    //context.platform.draw_region(&image, 0, 0, x, x, 0, None, 50, 10);

    draw_level_layer(&level.layer1);
    draw_shadows(&level);
    draw_objects(&level);
    draw_level_layer(&level.layer2);



    if draw_clip {
      let sprite = context.data.clips[current_sprite as usize][current_orientation][0];
      sprite::draw_sprite(sprite, Vec3i::new2(50, 50), 0);
    } else {
      sprite::draw_sprite(current_sprite as SpriteId, Vec3i::new2(50, 50), 0);
    }
    sprite::draw_sprite(1368, Vec3i::new2(200, 200), 0);

    for entity in context.game.entities.iter_mut() {
      entity.draw();
    }

    // draw camera
    context.platform.set_color(Color { r: 255, g: 0, b: 0, a: 255 });
    context.platform.fill_rect(context.game.camera.pos.x - 2,
                               context.game.camera.pos.y - 2,
                               context.game.camera.size.x + 2,
                               4);
    context.platform.fill_rect(context.game.camera.pos.x - 2,
                               context.game.camera.pos.y + context.game.camera.size.y - 2,
                               context.game.camera.size.x + 2,
                               4);
    context.platform.fill_rect(context.game.camera.pos.x - 2,
                               context.game.camera.pos.y - 2,
                               4,
                               context.game.camera.size.y + 2);
    context.platform.fill_rect(context.game.camera.pos.x + context.game.camera.size.x - 2,
                               context.game.camera.pos.y - 2,
                               4,
                               context.game.camera.size.y + 2);

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
