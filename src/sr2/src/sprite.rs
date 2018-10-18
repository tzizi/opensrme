use opensrme_common::*;
use super::types::*;

fn read_drawcommand(info: &Vec<i16>, pos: usize) -> (usize, DrawCommand) {
  return match info[pos] & 0xff {
    0 => (pos + 3, DrawCommand::Image {
      image_id: (info[pos] >> 8) as i8,
      start_x: info[pos + 1],
      start_y: info[pos + 2]
    }),

    1 => (pos + 1, DrawCommand::HFlip),

    2 => (pos + 1, DrawCommand::VFlip),

    3 => (pos + 3, DrawCommand::SetOffset {
      x: info[pos + 1],
      y: info[pos + 2]
    }),

    4 => (pos + 2, DrawCommand::DrawSprite(info[pos + 1])),

    5 => (pos + 4, DrawCommand::SetFrame {
      frame: info[pos + 1],
      total_time: info[pos + 2],
      frames: info[pos + 3]
    }),

    6 => (pos + 3, DrawCommand::SetColor(Color {
      a: 0xff,
      r: (info[pos + 1] & 0xff) as u8,
      g: (info[pos + 2] >> 8) as u8,
      b: (info[pos + 2] as u8) & 0xff
    })),

    10 => (pos + 3, DrawCommand::DrawShape {
      shape: DrawShape::Line,
      x: info[pos + 1],
      y: info[pos + 2]
    }),

    11 => (pos + 3, DrawCommand::DrawShape {
      shape: DrawShape::FillRect,
      x: info[pos + 1],
      y: info[pos + 2]
    }),

    12 => (pos + 3, DrawCommand::DrawShape {
      shape: DrawShape::DrawRect,
      x: info[pos + 1],
      y: info[pos + 2]
    }),

    13 => (pos + 3, DrawCommand::DrawShape {
      shape: DrawShape::FillArc,
      x: info[pos + 1],
      y: info[pos + 2]
    }),

    14 => (pos + 3, DrawCommand::DrawShape {
      shape: DrawShape::DrawArc,
      x: info[pos + 1],
      y: info[pos + 2]
    }),

    _ => (pos + 1, DrawCommand::Invalid)
  }
}

pub fn create_sprite(info: Vec<i16>, aabb: Vec<i16>) -> Sprite {
  let mut drawcommands = vec![];

  let mut i = 0;
  while i < info.len() {
    let (newpos, drawcommand) = read_drawcommand(&info, i);
    i = newpos;

    drawcommands.push(drawcommand);
  }

  return Sprite {
    aabb: aabb,
    draw: drawcommands
  };
}

pub fn calc_aabb(sprite: &Sprite, pos: Vec2i, flip: Flip) -> [i16; 4] {
  let mut x = sprite.aabb[0];
  let mut y = sprite.aabb[1];
  let mut width = sprite.aabb[2];
  let mut height = sprite.aabb[3];

  if flip & FLIP_H != 0 {
    x = -(x + width - 1);
  }

  if flip & FLIP_V != 0 {
    y = -(y + height - 1);
  }

  x += pos.x as i16;
  y += pos.y as i16;

  [x, y, x + width, y + height]
}

fn sub_draw_sprite(data: &DataContext, time: Time, platform: &mut Box<Platform>, images: &Vec<PaletteImage>, spriteid: SpriteId, pos: Vec2i, flip: Flip) {
  let sprite = &data.sprites[spriteid as usize];
  let aabb = calc_aabb(&sprite, pos, flip);

  let mut flip = flip;

  let mut startx = 0;
  let mut starty = 0;
  let mut hidden = false;

  //println!("{} {:?}\n  {:?}", spriteid, sprite, aabb);
  for command in sprite.draw.iter() {
    match *command {
      DrawCommand::Image {
        image_id,
        start_x,
        start_y
      } => {
        //println!("{:?}", context.images[image_id as usize]);
        platform.draw_region(
          &images[image_id as usize].image,
          start_x as IScalar,
          start_y as IScalar,
          (aabb[2] - aabb[0]) as IScalar,
          (aabb[3] - aabb[1]) as IScalar,
          flip,
          None,
          aabb[0] as IScalar,
          aabb[1] as IScalar
        );
      },
      DrawCommand::HFlip => {
        flip ^= FLIP_H;
      },
      DrawCommand::VFlip => {
        flip ^= FLIP_V;
      },
      DrawCommand::SetOffset { x, y } => {
        if (flip & FLIP_H) != 0 {
          startx = -x;
        } else {
          startx = x;
        }

        if (flip & FLIP_V) != 0 {
          starty = -y;
        } else {
          starty = y;
        }
      },
      DrawCommand::DrawSprite(new_spriteid) => {
        if !hidden {
          //println!("{} {}", new_spriteid, spriteid);
          if new_spriteid as SpriteId == spriteid as SpriteId {
            println!("Same sprite id: {}", spriteid);
          } else {
            sub_draw_sprite(data, time, platform, images, new_spriteid as SpriteId, Vec2i::new(pos.x + startx as i32, pos.y + starty as i32), flip);
          }
        }
      },
      DrawCommand::SetFrame {
        frame,
        total_time,
        frames
      } => {
        if (frame as u64) == (time / (total_time as u64)) % (frames as u64) {
          hidden = false;
        } else {
          hidden = true;
        }
      },
      DrawCommand::SetColor(color) => {
        platform.set_color(color)
      },
      DrawCommand::DrawShape {
        shape, x, y
      } => {
        let mut draw_x = pos.x + startx as IScalar;
        let mut draw_y = pos.y + starty as IScalar;

        if (flip & FLIP_H) != 0 {
          draw_x -= x as IScalar;
        }

        if (flip & FLIP_V) != 0 {
          draw_y -= y as IScalar;
        }
        match shape {
          DrawShape::FillRect => {
            platform.fill_rect(draw_x, draw_y, x.into(), y.into());
          },
          _ => {
          }
        }
      }
      _ => {
      }
    }
  }
}

pub fn draw_sprite(context: &mut Context, spriteid: SpriteId, pos: Vec2i, flip: Flip) {
  sub_draw_sprite(&context.data, context.time, &mut context.platform, &context.images, spriteid, pos, flip)
}
