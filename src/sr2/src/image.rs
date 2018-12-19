use super::*;
use byteorder::{BigEndian, ByteOrder};
use crc::crc32;

pub fn new(path: &str) -> PlatformId {
  let context = globals::get_context();

  context.platform.load_image_from_filename(&(*context.archive), path)
}

pub fn plte_chunk(size: u32, palette: &Palette) -> Vec<u8> {
  let mut result = vec![];

  result.push('P' as u8);
  result.push('L' as u8);
  result.push('T' as u8);
  result.push('E' as u8);

  for i in 0..size/3 {
    let color = palette.colors[i as usize];
    result.push(color.r);
    result.push(color.g);
    result.push(color.b);
  }

  result
}

pub fn write_chunk(data: Vec<u8>) -> Vec<u8> {
  let mut result = vec![];

  result.extend(&data[4..]);
  let crc = crc32::checksum_ieee(&data[..]);

  let mut buf = [0; 4];
  BigEndian::write_u32(&mut buf, crc);
  result.extend(&buf[..]);

  result
}

pub fn replace_image_palette<T: DataInputStream>(file: &mut T, palette: &Palette) -> io::Result<Vec<u8>> {
  let mut result = vec![];

  let header = file.read_amount_as_u8(8)?;
  result.extend(header);

  let mut finished = false;
  while !finished {
    let size_raw = file.read_amount_as_u8(4)?;
    let name_raw = file.read_amount_as_u8(4)?;

    result.extend(size_raw.clone());
    result.extend(name_raw.clone());

    if name_raw[0] == 'I' as u8 &&
      name_raw[1] == 'E' as u8 &&
      name_raw[2] == 'N' as u8 &&
      name_raw[3] == 'D' as u8 {
        finished = true;
        continue;
      }

    let size = BigEndian::read_u32(&size_raw[..]);
    let data = file.read_amount_as_u8(size as usize + 4)?;

    if name_raw[0] == 'P' as u8 &&
      name_raw[1] == 'L' as u8 &&
      name_raw[2] == 'T' as u8 &&
      name_raw[3] == 'E' as u8 {
        let chunk = plte_chunk(size, &palette);
        result.extend(write_chunk(chunk));

        finished = true;
        continue;
      } else {
        result.extend(data);
      }
  }

  let mut endbytes = vec![];
  file.read_to_end(&mut endbytes)?;
  result.extend(endbytes);

  Ok(result)
}

pub fn new_with_path_palette(path: &str, palette: &Palette) -> PlatformId {
  let context = globals::get_context();

  if let Ok(mut file) = context.archive.open_file(path) {
    if let Ok(data) = replace_image_palette(&mut file, palette) {
      return context.platform.load_image(&data[..]);
    }
  }

  0
}

pub fn load_image(image: ImageId, palette: PaletteId) -> PlatformId {
  let context = globals::get_context();

  let mut palette = palette;
  if palette == -1 {
    palette = 0;
  }

  let filename = &context.data.images[image as usize][..];
  let platform_id =
    match palette {
      0 => new(filename),
      _ => new_with_path_palette(filename, &context.data.palettes[palette as usize])
    };

  context.palette_images[palette as usize][image as usize] = platform_id;

  platform_id
}
