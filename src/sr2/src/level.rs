use opensrme_common::*;
use types::*;
use super::*;
use std::io;

fn read_levellayer<T: DataInputStream>(file: &mut T) -> io::Result<LevelLayer> {
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

fn read_levelobject<T: DataInputStream>(file: &mut T) -> io::Result<LevelObject> {
  let spriteid = file.read_short()?;
  let posx = file.read_short()?;
  let posy = file.read_short()?;

  Ok(LevelObject {
    pos: Vec3i::new2(posx as IScalar, posy as IScalar),
    sprite: spriteid as SpriteId
  })
}

fn read_levelentity<T: DataInputStream>(file: &mut T) -> io::Result<LevelEntity> {
  let entity_class_id = file.read_unsigned_byte()?;
  // if 0, and if player is female, set to 1

  let posx = file.read_short()?;
  let posy = file.read_short()?;
  let unk1 = file.read_short()?;
  let route_id = file.read_byte()?;

  println!("Class: {}\n  X: {}, Y: {}\n  Unk1: {}, Route: {}\n",
             entity_class_id, posx, posy, unk1, route_id);

  Ok(LevelEntity {
    class: entity_class_id as ClassId,
    pos: Vec3i::new2(posx as IScalar, posy as IScalar),
    unk1,
    route: route_id as RouteId
  })
}

fn read_routepart<T: DataInputStream>(file: &mut T) -> io::Result<RoutePart> {
  let x = file.read_short()? as FScalar;
  let y = file.read_short()? as FScalar;

  let pos = Vec3f::new2(x, y);

  let unk1 = file.read_unsigned_byte()?;

  Ok(RoutePart {
    pos,
    distance: 0.,
    unk1
  })
}

fn read_route<T: DataInputStream>(file: &mut T) -> io::Result<Route> {
  let route_parts_amt = file.read_byte()?;
  let mut parts: Vec<RoutePart> = vec![];

  for _i in 0..route_parts_amt {
    parts.push(read_routepart(file)?);
  }

  Ok(Route {
    parts
  })
}

pub fn read_level<T: DataInputStream>(file: &mut T) -> io::Result<Level> {
  file.skip(2)?;

  let mut levellayer1 = read_levellayer(file)?;
  let objects_amt = file.read_unsigned_short()?;
  let mut levellayer2 = read_levellayer(file)?;
  let mut objects = vec![];

  for _i in 0..objects_amt {
    objects.push(read_levelobject(file)?);
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
  let mut entities = vec![];
  println!("{} entities", entities_amt);
  for _i in 0..entities_amt {
    entities.push(read_levelentity(file)?);
  }

  let routes_amt = file.read_short()?;
  let mut routes = vec![];
  for _i in 0..routes_amt {
    let mut route = read_route(file)?;
    route::calc_route_distance(&mut route);
    routes.push(route);
  }

  Ok(Level {
    layer1: levellayer1,
    layer2: levellayer2,
    objects,
    tilesize: Vec3f::new2(tilesizex, tilesizey),
    tiledata_size: Vec3i::new2(tiledata_x as IScalar, tiledata_y as IScalar),
    tiledata,
    tile_gangdata,
    entities,
    routes
  })
}

pub fn get_level_from_levelid(levelid: LevelId) -> Level {
  let context = globals::get_context();

  if context.levels.contains_key(&levelid) {
    context.levels.get(&levelid).unwrap().clone()
  } else {
    let level = read_level(&mut context.archive.open_file(&context.data.levels[levelid as usize].path[..]).unwrap()).unwrap();
    context.levels.insert(levelid, level.clone());
    level
  }
}

pub fn load_entities(level: &Level) -> Vec<entity::Entity> {
  let mut context = globals::get_context();

  let mut entities = vec![];

  for level_entity in level.entities.iter() {
    let mut entity = entity::Entity::new(level_entity.class);
    entity.base.pos = level_entity.pos.into();

    entities.push(entity);
  }

  entities
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

pub fn draw_objects(level: &Level) {
  for i in 0..level.objects.len() {
    let object = &level.objects[i];

    if object.sprite == -1 {
      continue;
    }

    sprite::draw_sprite(object.sprite, object.pos, 0);
  }
}

pub fn tilepos_to_pos(tilepos: Vec3i) -> Vec3f {
  Vec3f::new2((tilepos.x as FScalar) * 24., (tilepos.y as FScalar) * 24.)
}

pub fn pos_to_tilepos(pos: Vec3f) -> Vec3i {
  Vec3i::new2((pos.x / 24.) as IScalar, (pos.y / 24.) as IScalar)
}

pub fn get_tiledata_for_tilepos(level: &Level, tilepos: Vec3i) -> i8 {
  if tilepos.x < 0 || tilepos.y < 0 || tilepos.x >= level.tiledata_size.x || tilepos.y >= level.tiledata_size.y {
    return 1;
  }

  return level.tiledata[(tilepos.y * level.tiledata_size.x + tilepos.x) as usize];
}

pub fn get_tiledata_for_pos(level: &Level, pos: Vec3f) -> i8 {
  get_tiledata_for_tilepos(level, pos_to_tilepos(pos))
}

pub fn pos_is_sidewalk(level: &Level, pos: Vec3f) -> bool {
  let tiledata = get_tiledata_for_pos(level, pos);

  tiledata == 9 || tiledata == 36
}
