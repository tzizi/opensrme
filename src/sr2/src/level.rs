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
    start: Vec3i::new2(startx, starty),
    tilesize: Vec3i::new2(tilesizex, tilesizey),
    size: Vec3i::new2(sizex, sizey),
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

  let tilesizex = ((file.readInt()? << 4) as FScalar) / 65536.;
  let tilesizey = ((file.readInt()? << 4) as FScalar) / 65536.;

  let unk_1 = file.read_byte()?;
  let mut tiledata_x = 0;
  let mut tiledata_y = 0;
  let mut tiledata = vec![];
  let mut tile_gangdata = vec![];

  if unk_1 > 0 {
    file.skip(4)?;

    tiledata_x = file.read_short()?;
    tiledata_y = file.read_short()?;
    tiledata = file.read_amount((tiledata_x * tiledata_y) as usize)?;
  }

  if unk_1 > 1 {
    file.skip(8)?;

    tile_gangdata = file.read_amount((tiledata_x * tiledata_y) as usize)?;
  }


  let entities_amt = file.read_unsigned_byte()?;
  println!("{} entities", entities_amt);
  for _i in 0..entities_amt {
    let entity_class_id = file.read_unsigned_byte()?;
    // if 0, and if player is female, set to 1

    let posx = file.read_short()?;
    let posy = file.read_short()?;
    let unk1 = file.read_short()?;
    let route_id = file.read_byte()?;

    println!("Class: {}\n  X: {}, Y: {}\n  Unk1: {}, Route: {}\n",
             entity_class_id, posx, posy, unk1, route_id);
  }

  let routes_amt = file.read_short()?;
  for i in 0..routes_amt {
    let route_parts_amt = file.read_byte()?;
    println!("{}", i);

    let mut parts: Vec<RoutePart> = vec![];
    let mut current_distance = 0.;

    for j in 0..route_parts_amt {
      let x = file.read_short()? as FScalar;
      let y = file.read_short()? as FScalar;

      let pos = Vec3f::new2(x, y);

      let unk1 = file.read_unsigned_byte()?;

      if j > 0 {
        current_distance += (pos - parts[(j as usize) - 1].pos).len2();
      }

      let routepart = RoutePart {
        pos,
        distance: current_distance,
        unk1
      };

      println!("{:?}", routepart);
      parts.push(routepart);
    }
  }

  Ok(Level {
    layer1: levellayer1,
    layer2: levellayer2,
    unk1,
    unk1_data,
    tilesizex,
    tilesizey,
    tiledata_size: Vec3i::new2(tiledata_x as IScalar, tiledata_y as IScalar),
    tiledata,
    tile_gangdata
  })
}

pub fn draw_level_layer(layer: &LevelLayer) {
  for x in 0..layer.size.x {
    for y in 0..layer.size.y {
      let tile = layer.tiles[((y * layer.size.x) + x) as usize];
      if tile >= 0 {
        sprite::draw_sprite(tile, Vec3i::new2(layer.start.x + x * layer.tilesize.x, layer.start.y + y * layer.tilesize.y), 0);
      }
    }
  }
}

pub fn draw_shadows(level: &Level) {
  let mut id: usize = 0;
  let ts_half = level.layer1.tilesize / 2;
  for y in 1..level.tiledata_size.y {
    id = (y * level.tiledata_size.x) as usize;
    let draw_y = (y * level.layer1.tilesize.y + ts_half.y) as IScalar;
    for x in 0..(level.tiledata_size.x - 1) {
      let pos = Vec3i::new2(x * level.layer1.tilesize.x + ts_half.x, draw_y);
      if level.tiledata[id] != 4 {
        if level.tiledata[id - level.tiledata_size.x as usize] == 4 { // wall above
          if level.tiledata[(id - level.tiledata_size.x as usize) + 1] == 4 { // wall above right
            if level.tiledata[id + 1] == 4 { // wall right
              // -+
              // x|
              sprite::draw_sprite(1361, pos, 0);
            } else {
              // --
              // x
              sprite::draw_sprite(1359, pos, 0);
            }
          } else { // bottom right corner
            // -
            // x
            sprite::draw_sprite(1360, pos, 0);
          }
        } else if level.tiledata[id + 1] == 4 { // wall right
          if level.tiledata[(id - level.tiledata_size.x as usize) + 1] == 4 { // wall above right
            //  |
            // x|
            sprite::draw_sprite(1357, pos, 0);
          } else {
            //
            // x|
            sprite::draw_sprite(1356, pos, 0);
          }
        } else if level.tiledata[(id - level.tiledata_size.x as usize) + 1] == 4 { // wall above right
          //  +
          // x
          sprite::draw_sprite(1358, pos, 0);
        }
      }

      id += 1;
    }
  }
}
