extern crate opensrme_common;
extern crate encoding;
extern crate rand;
extern crate byteorder;
extern crate crc;

mod types;
use types::*;
mod globals;
use globals::*;
mod sprite;
mod bin_all;
use bin_all::*;
mod level;
use level::*;
mod vehicle;
mod person;
mod entity;
mod route;
mod util;
mod input;
mod image;
mod text;
mod screen;
use screen::Screen;

#[macro_use]
use opensrme_common::*;

use std::io;

pub fn check(archive: &Archive) -> io::Result<bool> {
  let manifest = archive.get_manifest()?;

  Ok(check_manifest!(manifest,
                     "MIDlet-Name": "Saints Row 2"))
}

pub fn main(archive: Box<Archive>, _args: Vec<String>) {
  let mut platform = SDL2Platform::new("Saints Row 2", 800, 800);

  let splash = platform.load_image_from_filename(&(*archive), "Title.png");
  platform.set_color(Color { r: 0, g: 0, b: 0, a: 255 });
  platform.clear();
  platform.draw_region(splash, 0, 0, 240, 300, 0, None, 0, 10);
  platform.swap();

  let datacontext = read_bin_all(&(*archive)).unwrap();
  println!("{:?}", datacontext.levels);

  //let mut images = vec![];
  let mut palette_images = vec![];

  for _i in 0..datacontext.palettes.len() {
    let mut images = vec![];
    for _j in 0..datacontext.images.len() {
      images.push(0 as PlatformId);
    }

    palette_images.push(images);
  }

  let mut font_images = vec![];
  for _i in 0..datacontext.fonts.len() {
    font_images.push(0 as PlatformId);
  }

  //println!("{:?}", datacontext);

  /*for i in datacontext.effects.iter() {
    println!("{:?}", i);
  }*/

  let level = read_level(&mut archive.open_file("Street.lvl").unwrap()).unwrap();
  println!("{} {}", level.tilesize.x, level.tilesize.y);
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

  let context = Context {
    running: true,
    archive: archive,
    platform: Box::new(platform),
    time: instant_get_millis(),
    delta: 0,
    data: datacontext,
    palette_images,
    font_images,
    levels: std::collections::HashMap::new(),
    game: std::ptr::null_mut(),
    screen: None,
    input: input::InputContext::default()
  };

  set_context(context);
  let mut context = get_context();

  /*for i in context.data.images.iter() {
    println!("{}", i);
    context.images.push(PaletteImage {
      filename: i.clone(),
      image: image::new_with_path_palette(&i[..], &context.data.palettes[1])//platform.load_image_from_filename(&(*archive), &i[..])
    });
}*/

  for i in 0..context.data.fonts.len() {
    println!("Loading font #{}", i);
    text::load_font(i as FontId);
  }

  for i in 0..context.data.images.len() {
    println!("Loading image #{}", i);
    image::load_image(i as ImageId, 0);
  }

  let mut game = screen::GameScreen::new(0);
  game.init();

  //context.game = Some(game.clone());
  //context.screen = Some(game.clone());
  globals::set_game(game);

  //let image = context.platform.load_image_from_filename(archive, "Car_Police.png");

  let mut last_second = context.time;
  let mut fps = 0;
  while context.running {
    let lasttime = context.time;
    context.time = instant_get_millis();
    context.delta = context.time - lasttime;

    context.input.step();

    while let Some(event) = context.platform.poll_event() {
      if let Event::Quit = event {
        context.running = false;
        break;
      }

      context.input.process_platform_event(event);
    }

    if !context.running {
      break;
    }

    if let Some(ref mut screen) = context.screen {
      screen.step(context.delta);

      if !context.running {
        break;
      }

      screen.draw();
    }

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
