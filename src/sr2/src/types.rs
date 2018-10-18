use opensrme_common::*;

pub type Id = i32;
pub type SpriteId = i16;

#[derive(Debug)]
pub struct Palette {
  pub colors: Vec<Color>
}

#[derive(Debug)]
pub struct Font {
  pub name: String,
  pub palette: i32,
  pub height: i32,
  pub widths: Vec<Vec<i16>>,
  pub offsets: Vec<Vec<i16>>,
  pub size_addition: i16
}

#[derive(Debug)]
pub struct Language {
  pub strings: Vec<String>,
  pub fontid: i16
}

#[derive(Debug, Copy, Clone)]
pub enum DrawShape {
  Line,
  FillRect,
  DrawRect,
  FillArc,
  DrawArc
}

#[derive(Debug, Copy, Clone)]
pub enum DrawCommand {
  Invalid,

  // 0
  Image {
    image_id: i8,
    start_x: i16,
    start_y: i16
  },

  // 1
  HFlip,
  // 2
  VFlip,

  // 3
  SetOffset {
    x: i16,
    y: i16
  },

  // 4
  DrawSprite(i16),

  // 5
  SetFrame {
    frame: i16,
    total_time: i16,
    frames: i16
  },

  // 6
  SetColor(Color),

  // 10 = line
  // 11 = fillrect
  // 12 = drawrect
  // 13 = fillarc
  // 14 = drawarc
  DrawShape {
    shape: DrawShape,
    x: i16,
    y: i16
  },
}

#[derive(Debug)]
pub struct Sprite {
  pub aabb: Vec<i16>,
  pub draw: Vec<DrawCommand>
}


#[derive(Debug)]
pub struct PaletteImage {
  pub filename: String,
  pub image: PlatformId
}

#[derive(Debug)]
pub struct DataContext {
  pub palettes: Vec<Palette>,
  pub fonts: Vec<Font>,
  pub languages: Vec<Language>,
  pub images: Vec<String>,
  pub sprites: Vec<Sprite>
}

#[derive(Debug)]
pub struct LevelLayer {
  pub start: Vec2i,
  pub tilesize: Vec2i,
  pub size: Vec2i,
  pub tiles: Vec<i16>
}

#[derive(Debug)]
pub struct Level {
  pub layer1: LevelLayer,
  pub layer2: LevelLayer,
  pub unk1: u16,
  pub unk1_data: Vec<i16>
}

pub struct Context {
  pub platform: Box<Platform>,
  pub time: Time,
  pub delta: Time,
  pub data: DataContext,
  pub images: Vec<PaletteImage>,
  pub levels: Vec<Level>
}
