extern crate opensrme_common;
use opensrme_common::*;

#[derive(Debug)]
struct File_db {
  unk1: Vec<Vec<i32>>,
  unk2: Vec<i32>
}

#[derive(Debug)]
struct File_outfits {
}

#[derive(Debug)]
pub struct File_language {
  strings: Vec<String>
}

pub fn read_font_file(archive: &Archive) -> std::io::Result<()> {
  let mut contents = archive.open_file("fnt")?;
  contents.skip(3)?;

  let len1 = contents.read_byte()?;
  let mut arr1:Vec<Vec<i8>> = vec![];
  for i in 0..len1 {
    let len2 = contents.read_short()?;
    println!("2: {}", len2);
    //contents.skip(len2 as usize)?;
    arr1.push(contents.read_amount(len2 as usize)?);
    continue;
    println!("{:?}", arr1[i as usize]);
  }

  let len3 = contents.read_byte()?;
  for i in 0..len3 {
    let len4 = contents.read_short()?;
    println!("4: {}", len4);
    let read = contents.read_amount(len4 as usize)?;
    //contents.skip(len4 as usize)?;
    // 6: main offset (first drawn is shadow)
    // 7: shadow outfitid (or main if no shadow)
    // 8: whether or not a shadow is used
    // 9: main outfitid
    // 10: should always be 1, some kind of switch
    // 11: arr1 id (0, 1)

    println!("{:?}", read);
    println!("{}", read[11]); // arr1 id (0, 1)
    println!("{}", arr1[read[11] as usize][0xdf + 20]); // returns 9 (32 = 'A', 41-32 = 9)
  }

  Ok(())
}

pub fn read_language_file(archive: &Archive, language: &str) -> std::io::Result<File_language> {
  let mut contents = archive.open_file(language)?;
  contents.skip(3).unwrap();

  let mut out = File_language {
    strings: vec![]
  };

  for i in 0..84 {
    out.strings.push(String::from(""));
  }

  let len1 = contents.read_unsigned_short()?;

  for i in 0..len1 {
    let id = contents.read_unsigned_short()?;
    let len2 = contents.read_unsigned_short()?;
    out.strings[id as usize] = contents.read_utf()?;
  }

  Ok(out)
}

pub fn read_outfits(archive: &Archive) -> std::io::Result<()> {
  let mut contents = archive.open_file("outfits")?;
  contents.skip(3).unwrap();

  let len1 = contents.read_unsigned_short()?;
  for i in 0..len1 {
    let len2 = contents.read_unsigned_short()?; // id
    let len3 = contents.read_unsigned_short()?;
    // 4 = width (if 0, then getWidth)
    // 5 = height (if 0, then getWidth)
    let mut array:Vec<i8> = contents.read_amount(len3 as usize)?;

    let len4 = contents.read_unsigned_short()?;
    println!("{} {} {}", len2, len3, len4);
    let mut array2:Vec<i8> = contents.read_amount((len4 * 2) as usize)?;

    println!("{:?}", array);
    println!("{:?}\n", array2);
  }

  // more to do

  Ok(())
}

pub fn read_db(archive: &Archive) -> std::io::Result<()> {
  let mut contents = archive.open_file("db")?;
  contents.skip(3).unwrap();
  //println!("{}", contents.read_byte().unwrap());

  let len1 = contents.read_byte()?;
  println!("{}", len1);
  for i in 0..len1 {
    let len2 = contents.read_byte()?; // width?
    //println!("{}", len2);
    let mut array:Vec<i8> = vec![];
    for j in 0..len2 {
      array.push(contents.read_byte()?);
    }
    //println!("{:?}", array);

    let mut array2:Vec<i32> = vec![];
    let len3 = contents.read_short()?;
    //println!("{:?}", len3);

    for _j in 0..len3 {
      for k in 0..len2 {
        let mut n3:i32 = 0;

        let len4 = array[k as usize] as usize;
        //println!("{}", len4);

        // decode arbitrary number size
        for l in 0..len4 {
          n3 <<= 8;
          let mut n4:i32 = contents.read_byte()? as i32;
          if l > 0 {
            n4 &= 0xff;
          }
          n3 = n3 | n4;
        }

        array2.push(n3)
      }
    }

    println!("{:?}", array2);
  }

  Ok(())
}

pub fn check(archive: &Archive) -> std::io::Result<bool> {
  let manifest = archive.get_manifest()?;

  Ok(check_manifest!(manifest,
                     "MIDlet-Name": "LEGO Star Wars 2"))
}

pub fn main(archive: &Archive, args: Vec<String>) {
  //read_db(archive).unwrap();
  //read_outfits(archive).unwrap();
  let language = read_language_file(archive, "en").unwrap();

  let mut i = 0;
  for string in language.strings.iter() {
    println!("{} {}", i, string);
    i += 1;
  }

  read_font_file(archive).unwrap();
}

pub static GAME: Game = Game {
  name: "Lego Star Wars 2",

  main: main,
  check: check
};
