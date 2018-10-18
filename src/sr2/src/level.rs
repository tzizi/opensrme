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

  Ok(Level {
    layer1: levellayer1,
    layer2: levellayer2,
    unk1,
    unk1_data
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
