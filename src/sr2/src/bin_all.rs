use opensrme_common::*;
use super::types::*;
use super::sprite::*;
use std::io;
use encoding::{Encoding, DecoderTrap};
use encoding::all::ISO_8859_1;


fn read_palettes<T: DataInputStream>(file: &mut T) -> io::Result<Vec<Palette>> {
  let palette_amt = file.read_short()?;

  let mut palettes = vec![];

  for i in 0..palette_amt {
    let size = file.readInt()?;
    let elements = size / 3;

    let mut palette = Palette {
      colors: vec![]
    };

    for j in 0..elements {
      palette.colors.push(Color {
        r: file.read_unsigned_byte()?,
        g: file.read_unsigned_byte()?,
        b: file.read_unsigned_byte()?,
        a: 255
      });
    }

    palettes.push(palette);
  }

  Ok(palettes)
}

fn read_fonts<T: DataInputStream>(file: &mut T) -> io::Result<Vec<Font>> {
  let font_amt = file.read_short()?;

  let mut fonts = vec![];

  let mut font_names = vec![];
  let mut font_widths = vec![];
  let mut font_offsets = vec![];
  let mut font_size_additions = vec![];

  for i in 0..font_amt {
    font_names.push(file.read_utf()?);

    font_widths.push(vec![]);
    font_offsets.push(vec![]);
    font_size_additions.push(0);

    // always 1, style?
    let unk = file.read_short()?;

    for j in 0..unk {
      file.skip(4)?;

      font_widths[i as usize].push(vec![]);
      font_offsets[i as usize].push(vec![]);

      font_size_additions[i as usize] = file.read_short()?;

      for _k in 0..256 {
        font_offsets[i as usize][j as usize].push(file.read_short()?);
        font_widths[i as usize][j as usize].push(file.read_short()? - font_size_additions[i as usize]);
      }
    }
  }

  let font_amt = file.read_short()?;

  for _i in 0..font_amt {
    let font_id = file.readInt()? as usize;
    let palette_id = file.readInt()?;

    let mut font = Font {
      name: font_names[font_id].clone(),
      palette: palette_id,

      height: -1,
      widths: font_widths[font_id].clone(),
      offsets: font_offsets[font_id].clone(),
      size_addition: font_size_additions[font_id]
    };

    //println!("{:?}", font);

    fonts.push(font);
  }

  Ok(fonts)
}

fn read_latin1_string<T: DataInputStream>(file: &mut T) -> io::Result<String> {
  let string_len = file.read_short()?;

  let array = file.read_amount_as_u8(string_len as usize)?;
  if let Ok(result) = ISO_8859_1.decode(&array[..], DecoderTrap::Replace) {
    Ok(result)
  } else {
    Err(io::Error::new(io::ErrorKind::Other, "Encoding error"))
  }
  //println!("{} {:?}", j, language.strings[j as usize]);
}

fn read_strings<T: DataInputStream>(file: &mut T) -> io::Result<Vec<Language>> {
  let languages_amt = file.read_short()?;

  let languages = vec![];

  for i in 0..languages_amt {
    file.skip(4)?;

    let mut language = Language {
      strings: vec![],
      font_unk: 0
    };

    let strings_amt = file.read_short()?;
    for j in 0..strings_amt {
      language.strings.push(read_latin1_string(file)?);
    }

    // always 0
    language.font_unk = file.read_short()?;
  }

  Ok(languages)
}

fn read_sprites<T: DataInputStream>(file: &mut T, context: &mut DataContext) -> io::Result<()> {
  let images_amt = file.read_short()?;

  let mut sprite_info_offsets = vec![];
  sprite_info_offsets.push(0);
  let mut aabbs = vec![];

  for i in 0..images_amt {
    aabbs.push(vec! [
      file.read_short()?,
      file.read_short()?,
      file.read_short()?,
      file.read_short()?
    ]);

    sprite_info_offsets.push(file.read_short()?);
  }

  let mut sprite_infos = vec![];
  for i in 0..sprite_info_offsets[images_amt as usize] {
    sprite_infos.push(file.read_short()?);
  }

  let mut image_names = vec![];
  let image_names_amt = file.read_short()?;
  for i in 0..image_names_amt {
    image_names.push(file.read_utf()?);
  }

  context.images = image_names;

  let mut sprites = vec![];

  let mut last_i: i64 = -1;
  let mut sprite_id = 0;
  for i in sprite_info_offsets.iter() {
    sprite_id += 1;

    if last_i >= 0 {
      //println!("{} {}", last_i, *i);
      let newvec = sprite_infos[last_i as usize..*i as usize].to_vec();

      sprites.push(create_sprite(newvec, aabbs[sprite_id - 2].clone()));
    }

    last_i = *i as i64;
  }

  if false {
    // likely not needed? seems to return nil for newvec
    let newvec = sprite_infos[last_i as usize..sprite_infos.len()].to_vec();
    sprites.push(create_sprite(newvec, aabbs[sprite_id - 1].clone()));
  }

  /*sprite_id = 0;
  for sprite in sprites.iter() {
    println!("{}", sprite_id);

    for command in sprite.draw.iter() {
      println!("  {:?}", command);
    }

    sprite_id += 1;
  }*/

  context.sprites = sprites;

  Ok(())
}

fn read_clip<T: DataInputStream>(file: &mut T) -> io::Result<Clip> {
  let mut clip = vec![];

  let orientation_amt = file.read_short()?;
  for _i in 0..orientation_amt {
    let mut frames = vec![];

    let frames_amt = file.read_short()?;
    for _j in 0..frames_amt {
      frames.push(file.read_short()?);
    }

    clip.push(frames);
  }

  Ok(clip)
}

fn read_clips<T: DataInputStream>(file: &mut T) -> io::Result<Vec<Clip>> {
  let clips_amt = file.read_short()?;

  let mut clips = vec![];

  for _i in 0..clips_amt {
    clips.push(read_clip(file)?);
  }

  Ok(clips)
}

fn read_sounds<T: DataInputStream>(file: &mut T) -> io::Result<Vec<Sound>> {
  let sounds_amt = file.read_short()?;

  let mut sounds = vec![];

  for _i in 0..sounds_amt {
    let filename = file.read_utf()?;
    let mime = file.read_utf()?;
    let priority = file.readInt()?;
    let deferred_load = file.read_i8()? == 1;

    sounds.push(Sound {
      filename,
      mime,
      priority,
      deferred_load
    });
  }

  Ok(sounds)
}

fn read_items<T: DataInputStream>(file: &mut T) -> io::Result<Vec<Item>> {
  let items_amt = file.read_short()?;

  let mut items = vec![];

  for _i in 0..items_amt {
    let itemtype = file.readInt()?;
    let price = file.readInt()?;
    let increment = file.readInt()?;
    let maximum = file.readInt()?;
    let name = file.readInt()?;
    let description = file.readInt()?;
    let sprite = file.read_short()?;

    items.push(Item {
      itemtype,
      price,
      increment,
      maximum,
      name,
      description,
      sprite
    });
  }

  Ok(items)
}

fn read_quests<T: DataInputStream>(file: &mut T) -> io::Result<Vec<Quest>> {
  let quests_amt = file.read_short()?;

  let mut quests = vec![];

  for _i in 0..quests_amt {
    let giver = file.readInt()?;
    let is_mission_start = file.read_byte()? == 1;
    let giver_sprite = file.read_short()?;
    let name = file.readInt()?;
    let description = file.readInt()?;
    let levelid = file.readInt()?;

    quests.push(Quest {
      giver,
      is_mission_start,
      giver_sprite,
      name,
      description,
      levelid
    });
  }

  Ok(quests)
}

fn read_gangs<T: DataInputStream>(file: &mut T) -> io::Result<Vec<Gang>> {
  let gangs_amt = file.read_short()?;

  let mut gangs = vec![];

  for _i in 0..gangs_amt {
    let name = file.readInt()?;
    let sprite = file.read_short()?;
    let notoriety_bar_sprite = file.read_short()?;
    let default_notoriety = file.read_byte()?;
    let unk1 = file.readInt()?;

    gangs.push(Gang {
      name,
      sprite,
      notoriety_bar_sprite,
      default_notoriety,
      unk1
    });
  }

  Ok(gangs)
}

fn read_effects<T: DataInputStream>(file: &mut T) -> io::Result<Vec<Effect>> {
  let effects_amt = file.read_short()?;

  let mut effects = vec![];

  for _i in 0..effects_amt {
    let effect_type_id = file.readInt()?;
    let should_be_2 = file.readInt()?;
    let unk1 = file.readInt()?;
    let animation_time = file.read_unsigned_short()?;

    let effect_type = match effect_type_id {
      0 => EffectType::Clip(file.readInt()?),
      1 => {
        let mut spawners = vec![];

        let spawners_amt = file.read_short()?;

        for _j in 0..spawners_amt {
          let effect_id = file.readInt()?;
          let delay = file.read_unsigned_short()?;
          let position = [
            file.readInt()?,
            file.readInt()?,
            file.readInt()?
          ];

          spawners.push(EffectSpawner {
            effect: effect_id,
            delay,
            position
          });
        }

        EffectType::Spawner(spawners)
      },
      2 => {
        let effect_id = file.readInt()?;

        let modifiers_amt = file.read_short()?;
        let mut infos = vec![];

        let mut values = vec![];

        for _j in 0..modifiers_amt {
          let operation_id = file.readInt()?;
          let operation = match operation_id {
            0 => EffectModifierOperation::Linear,
            1 => EffectModifierOperation::MoveXY,
            2 => EffectModifierOperation::Curve,
            3 => EffectModifierOperation::Bounce,
            _ => {
              panic!("Unknown effect modifier operation {}", operation_id);
            }
          };

          let time_addition = file.readInt()?;

          let variable0 = file.readInt()?;
          let variable1 = file.readInt()?;

          infos.push(EffectModifierInfo {
            operation,
            time_addition,
            variable0,
            variable1
          });

          let mut subvalues = vec![];
          let subvalues_amt = file.read_short()?;

          for _k in 0..subvalues_amt*2 {
            subvalues.push((file.readInt()? as f32) / 65536.0);
          }

          values.push(subvalues);
        }

        EffectType::Modifier(EffectModifier {
          effect: effect_id,
          values,
          infos
        })
      },
      3 => {
        let color = Color::from_bgr(file.readInt()? as u32);
        let size = file.read_unsigned_byte()?;

        EffectType::Square {
          color,
          size
        }
      },
      4 => {
        let color = Color::from_bgr(file.readInt()? as u32);
        let size = file.readInt()?;

        EffectType::Line {
          color,
          size
        }
      },
      _ => {
        panic!("Unknown effect type {}", effect_type_id);
      }
    };

    effects.push(Effect {
      should_be_2,
      unk1,
      animation_time,
      effect_type
    });
  }

  Ok(effects)
}

fn read_classes<T: DataInputStream>(file: &mut T) -> io::Result<Vec<EntityClass>> {
  let classes_amt = file.read_short()?;

  let mut classes = vec![];

  for _i in 0..classes_amt {
    let entity_type = file.readInt()?;
    let clip = file.readInt()?;
    let health = file.read_short()?;
    let unk1 = file.readInt()?;
    let width = (file.readInt()? as FScalar) / 65536.0;
    let height = (file.readInt()? as FScalar) / 65536.0;
    let unk2 = file.readInt()?;
    let unk3 = file.readInt()?;

    classes.push(EntityClass {
      entity_type,
      clip,
      health,
      unk1,
      width,
      height,
      unk2,
      unk3
    });
  }

  Ok(classes)
}

fn read_weapons<T: DataInputStream>(file: &mut T) -> io::Result<Vec<Weapon>> {
  let weapons_amt = file.read_short()?;

  let mut weapons = vec![];

  for _i in 0..weapons_amt {
    let item = file.readInt()?;
    let weapon_type_id = file.readInt()?;

    let weapon_type = match weapon_type_id {
      0 => WeaponType::Melee,
      1 => WeaponType::Pistol,
      2 => WeaponType::SMG,
      3 => WeaponType::Assault,
      4 => WeaponType::Heavy,
      _ => WeaponType::Unknown
    };

    let damage = file.read_short()?;
    let cooldown = file.read_short()?;
    let bullet_area = (file.readInt()? as FScalar) / 65536.0;
    let item_increment = file.read_byte()?;
    let sound = file.readInt()?;

    weapons.push(Weapon {
      item,
      weapon_type,
      damage,
      cooldown,
      bullet_area,
      item_increment,
      sound
    });
  }

  Ok(weapons)
}

fn read_vehicles<T: DataInputStream>(file: &mut T) -> io::Result<Vec<Vehicle>> {
  let vehicles_amt = file.read_short()?;

  let mut vehicles = vec![];

  for _i in 0..vehicles_amt {
    let mut vehicle = Vehicle {
      gears: [0.; 7]
    };

    for j in 0..7 {
      vehicle.gears[j] = (file.readInt()? as FScalar) / 65536.0;
    }

    vehicles.push(vehicle);
  }

  Ok(vehicles)
}

fn read_businesses<T: DataInputStream>(file: &mut T) -> io::Result<Vec<Business>> {
  let businesses_amt = file.read_short()?;

  let mut businesses = vec![];

  for _i in 0..businesses_amt {
    businesses.push(Business {
      sprite: file.read_short()?
    });
  }

  Ok(businesses)
}

fn read_robbery_items<T: DataInputStream>(file: &mut T) -> io::Result<Vec<RobberyItem>> {
  let robbery_items_amt = file.read_short()?;

  let mut robbery_items = vec![];

  for _i in 0..robbery_items_amt {
    let worth = file.readInt()?;

    let rotations_amt = file.read_short()?;
    let mut rotations = vec![];

    for _j in 0..rotations_amt {
      let sprite = file.read_short()?;

      let mut tiledata: [i32; 5] = [
        file.readInt()?,
        file.readInt()?,
        file.readInt()?,
        file.readInt()?,
        file.readInt()?
      ];

      rotations.push(RobberyItemRotation {
        sprite,
        tiledata
      });
    }

    robbery_items.push(RobberyItem {
      worth,
      rotations
    });
  }

  Ok(robbery_items)
}

fn read_conversations<T: DataInputStream>(file: &mut T) -> io::Result<Vec<Conversation>> {
  let conversations_amt = file.read_short()?;

  let mut conversations = vec![];

  for _i in 0..conversations_amt {
    let can_redraw = file.read_boolean()?;
    let tutorial = file.read_boolean()?;

    let items_amt = file.read_short()?;
    let mut items = vec![];
    for _j in 0..items_amt {
      let name = file.readInt()?;
      let text = file.readInt()?;
      let sprite = file.read_short()?;

      items.push(ConversationItem {
        name,
        text,
        sprite
      });
    }

    conversations.push(Conversation {
      can_redraw,
      tutorial,
      items
    });
  }

  Ok(conversations)
}

fn read_levels<T: DataInputStream>(file: &mut T) -> io::Result<Vec<LevelInfo>> {
  let levels_amt = file.read_short()?;

  let mut levels = vec![];

  for _i in 0..levels_amt {
    let path = file.read_utf()?;

    let images_amt = file.read_short()?;
    let mut images = vec![];
    for _j in 0..images_amt {
      let image = file.readInt()?;
      let palette = file.readInt()?;

      images.push(LevelImageInfo {
        image,
        palette
      });
    }

    levels.push(LevelInfo {
      path,
      images
    });
  }

  Ok(levels)
}

pub fn read_bin_all(archive: &Archive) -> io::Result<DataContext> {
  let mut context = DataContext {
    palettes: vec![],
    fonts: vec![],
    languages: vec![],
    images: vec![],
    sprites: vec![],
    clips: vec![],
    sounds: vec![],
    items: vec![],
    quests: vec![],
    gangs: vec![],
    effects: vec![],
    classes: vec![],
    weapons: vec![],
    vehicles: vec![],
    businesses: vec![],
    robbery_items: vec![],
    conversations: vec![],
    levels: vec![]
  };

  let mut contents = archive.open_file("bin.all")?;

  context.palettes = read_palettes(&mut contents).unwrap();
  context.fonts = read_fonts(&mut contents).unwrap();
  context.languages = read_strings(&mut contents).unwrap();
  read_sprites(&mut contents, &mut context).unwrap();
  context.clips = read_clips(&mut contents).unwrap();
  context.sounds = read_sounds(&mut contents).unwrap();
  context.items = read_items(&mut contents).unwrap();
  context.quests = read_quests(&mut contents).unwrap();
  context.gangs = read_gangs(&mut contents).unwrap();
  context.effects = read_effects(&mut contents).unwrap();
  context.classes = read_classes(&mut contents).unwrap();
  context.weapons = read_weapons(&mut contents).unwrap();
  context.vehicles = read_vehicles(&mut contents).unwrap();
  context.businesses = read_businesses(&mut contents).unwrap();
  context.robbery_items = read_robbery_items(&mut contents).unwrap();
  context.conversations = read_conversations(&mut contents).unwrap();
  context.levels = read_levels(&mut contents).unwrap();

  Ok(context)
}
