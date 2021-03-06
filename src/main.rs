// Use system allocator for non-debug builds
#[cfg(not(debug_assertions))]
use std::alloc::System;
#[global_allocator]
#[cfg(not(debug_assertions))]
static GLOBAL: System = System;

extern crate opensrme_common;

extern crate opensrme_sr2;

use opensrme_common::*;

fn main() {
  opensrme_common::instant_init();

  let args: Vec<String> = std::env::args().collect();
  if args.len() < 2 {
    println!("Provide archive");
    return;
  }

  let games = vec![
    &opensrme_sr2::GAME
  ];

  let file = &args[1];
  let archive:FilesystemArchive = FilesystemArchive::new(file);

  for game in games.iter() {
    let ok = match (game.check)(&archive) {
      Ok(value) => value,
      _ => false
    };

    if ok {
      println!("Playing {}", game.name);
      return (game.main)(Box::new(archive), args[2..].to_vec());
    }
  }

  println!("No supported game found");
}
