extern crate opensrme_common;
extern crate encoding;
extern crate rand;
extern crate byteorder;
extern crate crc;
extern crate nalgebra;
extern crate ncollide2d;

mod types;
use types::*;
mod globals;
mod sprite;
mod bin_all;
use bin_all::*;
mod level;
use level::*;
mod vehicle;
mod person;
mod collision;
mod entity;
mod controller;
mod route;
mod util;
mod input;
mod image;
mod text;
mod dialog;
mod screen;
use screen::Screen;

#[macro_use]
use opensrme_common::*;

use std::io;

pub fn check(archive: &Archive) -> io::Result<bool> {
  let files = archive.list_dir(".")?;
  for file in files {
    if file == "bin.all" {
      return Ok(true);
    }
  }

  Ok(false)
}

fn draw_splash(archive: &Box<Archive>, platform: &mut Platform) {
  let splash = platform.load_image_from_filename(&(**archive), "Title.png");
  platform.set_color(Color { r: 0, g: 0, b: 0, a: 255 });
  platform.clear();

  let size = platform.get_size();
  let scaley = size.y as FScalar / 300.;
  platform.scale(scaley);

  platform.draw_region(splash, 0, 0, 240, 300, 0, None, (((size.x as FScalar / scaley) - 240.) / 2.) as IScalar, 0);
  platform.swap();
}

pub fn main(archive: Box<Archive>, _args: Vec<String>) {
  let mut platform = SDL2Platform::new("Saints Row 2", 800, 800);

  draw_splash(&archive, &mut platform);

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

  let context = globals::Context {
    running: true,
    archive: archive,
    platform: Box::new(platform),
    realtime: instant_get_millis(),
    time: 1,
    delta: 0,
    data: datacontext,
    palette_images,
    font_images,
    levels: std::collections::HashMap::new(),
    game: std::ptr::null_mut(),
    screen: None,
    input: input::InputContext::default()
  };

  globals::set_context(context);
  let mut context = globals::get_context();

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

  /*for i in 0..context.data.images.len() {
    if i == 35 { // resource.max
      continue;
    }

    println!("Loading image #{}", i);
    image::load_image(i as ImageId, 0);
  }*/

  let mut game = screen::GameScreen::new(0);
  game.init();

  //context.game = Some(game.clone());
  //context.screen = Some(game.clone());
  globals::set_game(game);

  //let image = context.platform.load_image_from_filename(archive, "Car_Police.png");

  let mut last_second = context.realtime;
  let mut fps = 0;
  while context.running {
    let lasttime = context.realtime;
    context.realtime = instant_get_millis();
    context.delta = (context.realtime - lasttime) * 1;
    context.time += context.delta;

    context.input.step();

    let mut newsize: Option<Vec3i> = None;
    while let Some(event) = context.platform.poll_event() {
      if let Event::Quit = event {
        context.running = false;
        break;
      }

      context.input.process_platform_event(event);

      if let Event::Resize(new_size) = event {
        newsize = Some(new_size);
      }
    }

    if !context.running {
      break;
    }

    if let Some(ref mut screen) = context.screen {
      if let Some(newsize) = newsize {
        screen.set_size(newsize);
      }

      screen.step(context.delta);

      if !context.running {
        break;
      }

      screen.draw();
    }

    context.platform.swap();

    if context.realtime - last_second >= 1000 {
      last_second = context.realtime;
      //println!("{}", fps);
      context.platform.set_title(&format!("Saints Row 2 ({} FPS)", fps)[..]);
      fps = 0;
    } else {
      fps += 1;
    }

    let mut sleep = 16;
    let millis = instant_get_millis() - context.realtime;
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
