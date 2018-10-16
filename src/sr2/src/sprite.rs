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

    4 => (pos + 2, DrawCommand::DrawSprite(pos as i16 + 1)),

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
