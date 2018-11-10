use opensrme_common::*;

pub type Id = i32;
pub type SpriteId = i16;
pub type TextId = Id;
pub type ClipId = Id;
pub type EffectId = Id;
pub type ClassId = Id;
pub type RouteId = Id;

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
  DrawSprite(SpriteId),

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

// [orientation][frames]
pub type Clip = Vec<Vec<SpriteId>>;

#[derive(Debug)]
pub struct Sound {
  pub filename: String,
  pub mime: String,
  pub priority: i32,
  pub deferred_load: bool
}

#[derive(Debug)]
pub struct Item {
  // 0: weapon
  // 1: food
  // 2: addon (gear, durability, colors)
  pub itemtype: i32,
  pub price: i32,
  pub increment: i32,
  pub maximum: i32,
  pub name: TextId,
  pub description: TextId,
  pub sprite: SpriteId
}

#[derive(Debug)]
pub struct Quest {
  // 1 = ?
  // 2 = active
  // 4 = complete
  //pub state: i32
  pub giver: TextId,
  pub is_mission_start: bool,
  pub giver_sprite: SpriteId,
  pub name: TextId,
  pub description: TextId,
  pub levelid: i32
}

#[derive(Debug)]
pub struct Gang {
  pub name: TextId,
  pub sprite: SpriteId,
  pub notoriety_bar_sprite: SpriteId,
  pub default_notoriety: i8,
  pub unk1: i32
}

#[derive(Debug)]
pub struct EffectSpawner {
  pub effect: EffectId,
  pub delay: u16,
  pub position: [i32; 3]
}

#[derive(Debug)]
pub enum EffectModifierOperation {
  // values[0] = multiplier (* secs_elapsed)
  // values[1] = offset
  // variable0 = (values[0] * seconds) + values[1];
  Linear,
  // values[0] = angle
  // values[1] = multiplier (* secs_elapsed)
  // variable0: x
  // variable1: y
  // variable0 = (values[1] * seconds) * cos(values[0]);
  // variable1 = (values[1] * seconds) * sin(values[0]);
  MoveXY,
  // values[0] = width of curve (or total time)
  // values[1] = height of curve (or intensity)
  // values[2] = curve offset (positive = shorter, negative = delay before the curve)
  // values[3] = additional offset (adds to the final result)
  // variable0 = (4*(values[1]/values[0]) - (4*(values[1]/values[0])/values[0]) * (values[2] + seconds)) * (values[2] + seconds) + values[3]
  Curve,
  // values[0] speed (amount of bounces, 1 = one bounce per pi, 2 = pi/2)
  // values[1] intensity (how high)
  // values[2] decay/gravity (negative values decay, positive values increase, the larger the value in either direction, the more intense the decay/increase)
  // variable0 = (values[1] * abs(sin(values[0] * seconds))) * exp(values[2] * seconds);
  Bounce
}

#[derive(Debug)]
pub struct EffectModifierInfo {
  pub operation: EffectModifierOperation,
  pub time_addition: i32, // always 0?
  // 0: x
  // 1: y,
  // 2: y subtraction (y - this)
  pub variable0: i32,
  pub variable1: i32
}

#[derive(Debug)]
pub struct EffectModifier {
  pub effect: EffectId,
  pub values: Vec<Vec<f32>>,
  pub infos: Vec<EffectModifierInfo>
}

#[derive(Debug)]
pub enum EffectType {
  Clip(ClipId), // 0
  Spawner(Vec<EffectSpawner>), // 1
  Modifier(EffectModifier), // 2
  Square { // 3
    color: Color,
    size: u8
  },
  Line { //4
    color: Color,
    size: i32
  }
}

#[derive(Debug)]
pub struct Effect {
  pub should_be_2: i32,
  pub unk1: i32,
  pub animation_time: u16,
  pub effect_type: EffectType
}

#[derive(Debug)]
pub struct EntityClass {
  pub entity_type: i32,
  pub clip: ClipId,
  pub health: i16,
  pub unk1: i32,
  pub width: FScalar,
  pub height: FScalar,
  pub unk2: i32,
  pub unk3: i32
}

#[derive(Debug)]
pub struct DataContext {
  pub palettes: Vec<Palette>,
  pub fonts: Vec<Font>,
  pub languages: Vec<Language>,
  pub images: Vec<String>,
  pub sprites: Vec<Sprite>,
  pub clips: Vec<Clip>,
  pub sounds: Vec<Sound>,
  pub items: Vec<Item>,
  pub quests: Vec<Quest>,
  pub gangs: Vec<Gang>,
  pub effects: Vec<Effect>,
  pub classes: Vec<EntityClass>
}

#[derive(Debug)]
pub struct LevelLayer {
  pub start: Vec3i,
  pub tilesize: Vec3i,
  pub size: Vec3i,
  pub tiles: Vec<i16>
}

#[derive(Debug)]
pub struct LevelObject {
  pub pos: Vec3i,
  pub sprite: SpriteId
}

#[derive(Debug)]
pub struct RoutePart {
  // 0, 1, 2
  pub pos: Vec3f,
  // 3
  pub distance: f64,
  pub unk1: u8
}

#[derive(Debug)]
pub struct Route {
  pub parts: Vec<RoutePart>
}

#[derive(Debug)]
pub struct LevelEntity {
  pub class: ClassId,
  pub pos: Vec3i,
  pub unk1: i16,
  pub route: RouteId
}

#[derive(Debug)]
pub struct Level {
  pub layer1: LevelLayer,
  pub layer2: LevelLayer,
  pub objects: Vec<LevelObject>,
  pub tilesizex: FScalar,
  pub tilesizey: FScalar,
  pub tiledata_size: Vec3i,
  pub tiledata: Vec<i8>,
  pub tile_gangdata: Vec<i8>,

  pub entities: Vec<LevelEntity>,
  pub routes: Vec<Route>
}

pub struct Context {
  pub platform: Box<Platform>,
  pub time: Time,
  pub delta: Time,
  pub data: DataContext,
  pub images: Vec<PaletteImage>,
  pub levels: Vec<Level>
}
