use super::*;

pub fn load_font(fontid: FontId) -> PlatformId {
  let context = globals::get_context();
  let font = &context.data.fonts[fontid as usize];
  let path = font.name.clone() + ".png";

  let imageid = if font.palette > 0 {
    let palette = &context.data.palettes[font.palette as usize];
    image::new_with_path_palette(&path[..], palette)
  } else {
    image::new(&path[..])
  };

  context.font_images[fontid as usize] = imageid;

  let font_size = context.platform.get_image_size(imageid).unwrap();

  let context = globals::get_context();
  context.data.fonts[fontid as usize].height = font_size.y as i16;

  imageid
}

pub fn text_size(fontid: FontId, text: &str) -> Vec3i {
  let context = globals::get_context();
  let font = &context.data.fonts[fontid as usize];
  let widths = &font.widths[0];
  let chars: Vec<char> = text.chars().collect();

  let mut size = Vec3i::default();
  size.y = font.height as IScalar;

  for i in 0..text.len() {
    let charid = chars[i] as usize;

    size.x += widths[charid] as IScalar;
  }

  size
}

pub fn word_wrap(fontid: FontId, text: &str, limit: IScalar) -> Vec<String> {
  let context = globals::get_context();
  let font = &context.data.fonts[fontid as usize];
  let widths = &font.widths[0];

  let mut result = vec![];

  let chars: Vec<char> = text.chars().collect();
  let mut i = 0;

  let mut start = 0;
  let mut end = 0;
  let mut current_width = 0;

  while i <= chars.len() {
    let mut wrap = false;

    let ch = if i < chars.len() {
      chars[i]
    } else {
      '\n'
    };

    if ch == '\n' {
      wrap = true;
    } else {
      if ch == ' ' {
        end = i;
      }

      current_width += widths[ch as usize];
      if current_width as IScalar > limit {
        wrap = true;
      }
    }

    if wrap {
      if start == end || ch == '\n' {
        end = i;
      }

      i = end;

      if ch == '\n' || ch == ' ' {
        i += 1;
      }

      let new_string: String = text.chars().skip(start).take(end-start).collect();
      result.push(new_string.trim().to_string());

      start = i;
      end = i;
      current_width = 0;
    } else {
      i += 1;
    }
  }

  result
}

pub fn draw_text(fontid: FontId, text: &str, pos: Vec3i) -> Vec3i {
  let context = globals::get_context();
  let font = &context.data.fonts[fontid as usize];
  let widths = &font.widths[0];
  let offsets = &font.offsets[0];
  let image = context.font_images[fontid as usize];
  let chars: Vec<char> = text.chars().collect();

  let mut x = pos.x;

  for i in 0..text.len() {
    let charid = chars[i] as usize;

    context.platform.draw_region(
      image,
      offsets[charid] as IScalar + font.size_addition as IScalar, 0,
      widths[charid] as IScalar, font.height as IScalar,
      0,
      None,
      x, pos.y);

    x += widths[charid] as IScalar;
  }

  Vec3i::new2(x, font.height.into())
}
