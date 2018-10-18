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

    let styles = file.read_short()?;

    for j in 0..styles {
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
      fontid: 0
    };

    let strings_amt = file.read_short()?;
    for j in 0..strings_amt {
      language.strings.push(read_latin1_string(file)?);
    }

    language.fontid = file.read_short()?;
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
      println!("{} {}", last_i, *i);
      let newvec = sprite_infos[last_i as usize..*i as usize].to_vec();

      sprites.push(create_sprite(newvec, aabbs[sprite_id - 2].clone()));

      /*
        let mut j = 0;
        let mut flags = 0;
        // 1 = mirror/fliph
        // 2 = mirror_rot180/flipv
        // (3 = rot180 without mirroring)
        // 4 = don't show (blink)

        let mut x_add = 0;
        let mut y_add = 0;

        while j < newvec.len() {
          let current = j;
          j += 1;

          match newvec[current] & 0xff {
            0 => {
              println!("  image id: {} {:?}", newvec[current] >> 8, image_names[(newvec[current] >> 8) as usize]);

              if j + 2 <= newvec.len() {
                println!("  crop: {} {}", newvec[j], newvec[j + 1]);
              }

              j += 2;
            },
            1 => {
              flags ^= 1;
            },
            2 => {
              flags ^= 2;
            },
            3 => {
              let mut ournum = 0;

              if (flags & 1) != 0 {
                ournum = -sprite_infos[j];
                j += 1;
              } else {
                ournum = sprite_infos[j];
                j += 1;
              }

              x_add = ournum;

              let mut ournum1 = 0;
              if (flags & 2) != 0 {
                ournum1 = -sprite_infos[j];
                j += 1;
              } else {
                ournum1 = sprite_infos[j];
                j += 1;
              }

              y_add = ournum1;
            },
            4 => {
              if (flags & 4) != 0 {
                j += 1;
                continue;
              }


            },
            _ => {
            }
          }
       }*/

      //println!("{:?} {} {:?}", newvec, newvec[0] & 0xff, image_names[(newvec[0] >> 8) as usize]);
    }

    last_i = *i as i64;
  }

  if false {
    // likely not needed? seems to return nil for newvec
    let newvec = sprite_infos[last_i as usize..sprite_infos.len()].to_vec();
    sprites.push(create_sprite(newvec, aabbs[sprite_id - 1].clone()));
  }

  sprite_id = 0;
  for sprite in sprites.iter() {
    println!("{}", sprite_id);

    for command in sprite.draw.iter() {
      println!("  {:?}", command);
    }

    sprite_id += 1;
  }

  context.sprites = sprites;

  Ok(())
}

pub fn read_bin_all(archive: &Archive) -> io::Result<DataContext> {
  let mut context = DataContext {
    palettes: vec![],
    fonts: vec![],
    languages: vec![],
    images: vec![],
    sprites: vec![]
  };

  let mut contents = archive.open_file("bin.all")?;

  context.palettes = read_palettes(&mut contents).unwrap();
  context.fonts = read_fonts(&mut contents).unwrap();
  context.languages = read_strings(&mut contents).unwrap();
  read_sprites(&mut contents, &mut context).unwrap();

  Ok(context)
}
