use opensrme_common::*;
use types::*;
use super::*;
use std::io;

pub fn read_levellayer<T: DataInputStream>(file: &mut T) -> io::Result<LevelLayer> {
  let startx:IScalar = file.read_short()? as IScalar;
  let starty:IScalar = file.read_short()? as IScalar;

  let tilesizex:IScalar = file.read_short()? as IScalar;
  let tilesizey:IScalar = file.read_short()? as IScalar;

  let sizex:IScalar = file.read_short()? as IScalar;
  let sizey:IScalar = file.read_short()? as IScalar;

  Ok(LevelLayer {
    start: Vec2i::new(startx, starty),
    tilesize: Vec2i::new(tilesizex, tilesizey),
    size: Vec2i::new(sizex, sizey),
    tiles: vec![]
  })
}

pub fn read_level<T: DataInputStream>(file: &mut T) -> io::Result<Level> {
  file.skip(2)?;

  let mut levellayer1 = read_levellayer(file)?;
  let unk1 = file.read_unsigned_short()?;
  let mut levellayer2 = read_levellayer(file)?;
  let mut unk1_data = vec![];

  for _i in 0..unk1*3 {
    unk1_data.push(file.read_short()?);
  }

  for _i in 0..(levellayer1.size.x * levellayer1.size.y) {
    levellayer1.tiles.push(file.read_short()?);
  }

  for _i in 0..(levellayer2.size.x * levellayer2.size.y) {
    levellayer2.tiles.push(file.read_short()?);
  }

  let unk2 = file.readInt()? << 4;
  let unk3 = file.readInt()? << 4;

  let unk_1 = file.read_byte()?;
  let mut tiledata_x = 0;
  let mut tiledata_y = 0;
  let mut tiledata = vec![];

  if unk_1 > 0 {
    file.skip(4)?;

    tiledata_x = file.read_short()?;
    tiledata_y = file.read_short()?;
    tiledata = file.read_amount((tiledata_x * tiledata_y) as usize)?;
  }

  Ok(Level {
    layer1: levellayer1,
    layer2: levellayer2,
    unk1,
    unk1_data,
    unk2,
    unk3,
    tiledata_size: Vec2i::new(tiledata_x as IScalar, tiledata_y as IScalar),
    tiledata
  })
}

pub fn draw_level_layer(context: &mut Context, layer: &LevelLayer) {
  for x in 0..layer.size.x {
    for y in 0..layer.size.y {
      let tile = layer.tiles[((y * layer.size.x) + x) as usize];
      if tile >= 0 {
        sprite::draw_sprite(context,
                            tile, Vec2i::new(layer.start.x + x * layer.tilesize.x, layer.start.y + y * layer.tilesize.y), 0);
      }
    }
  }
}

pub fn draw_shadows(context: &mut Context, level: &Level) {
  let mut id: usize = 0;
  let ts_half = level.layer1.tilesize / 2;
  for y in 1..level.tiledata_size.y {
    id = (y * level.tiledata_size.x) as usize;
    let draw_y = (y * level.layer1.tilesize.y + ts_half.y) as IScalar;
    for x in 0..(level.tiledata_size.x - 1) {
      let pos = Vec2i::new(x * level.layer1.tilesize.x + ts_half.x, draw_y);
      if level.tiledata[id] != 4 {
        if level.tiledata[id - level.tiledata_size.x as usize] == 4 { // wall above
          if level.tiledata[(id - level.tiledata_size.x as usize) + 1] == 4 { // wall above right
            if level.tiledata[id + 1] == 4 { // wall right
              // -+
              // x|
              sprite::draw_sprite(context, 1361, pos, 0);
            } else {
              // --
              // x
              sprite::draw_sprite(context, 1359, pos, 0);
            }
          } else { // bottom right corner
            // -
            // x
            sprite::draw_sprite(context, 1360, pos, 0);
          }
        } else if level.tiledata[id + 1] == 4 { // wall right
          if level.tiledata[(id - level.tiledata_size.x as usize) + 1] == 4 { // wall above right
            //  |
            // x|
            sprite::draw_sprite(context, 1357, pos, 0);
          } else {
            //
            // x|
            sprite::draw_sprite(context, 1356, pos, 0);
          }
        } else if level.tiledata[(id - level.tiledata_size.x as usize) + 1] == 4 { // wall above right
          //  +
          // x
          sprite::draw_sprite(context, 1358, pos, 0);
        }
      }

      id += 1;
    }
  }
}
