use super::*;
use ncollide2d::query::PointQuery;
use std::sync::Arc;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Shape {
  Rect(Vec3i),
  Circle(IScalar)
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ShapeInfo {
  pub shape: Shape,
  pub weight: IScalar
}

// TODO: weight
#[derive(Clone)]
pub struct PhysicalObject {
  pub nashape: Arc<ncollide2d::shape::Shape<FScalar>>,
  pub shape: Shape,
  pub isometry: nalgebra::Isometry2<FScalar>,
  pub weight: FScalar
}

fn vec_to_navec(vec: Vec3f) -> nalgebra::Vector2<FScalar> {
  nalgebra::Vector2::new(
    vec.x,
    vec.y
  )
}

fn vec_to_napoint(vec: Vec3f) -> nalgebra::Point2<FScalar> {
  nalgebra::Point2::new(
    vec.x,
    vec.y
  )
}

fn navec_to_vec(vec: nalgebra::Vector2<FScalar>) -> Vec3f {
  Vec3f::new2(
    vec.x,
    vec.y
  )
}

impl PhysicalObject {
  pub fn new_from_info(info: ShapeInfo) -> Self {
    let shape: Arc<ncollide2d::shape::Shape<FScalar>> = match info.shape {
      Shape::Rect(vec) => {
        Arc::new(ncollide2d::shape::Cuboid::new(vec_to_navec(vec.into())))
      },
      Shape::Circle(radius) => {
        Arc::new(ncollide2d::shape::Ball::new(radius.into()))
      }
    };

    PhysicalObject {
      nashape: shape,
      shape: info.shape,
      isometry: nalgebra::Isometry2::<FScalar>::identity(),
      weight: 1. / info.weight as FScalar
    }
  }

  pub fn clone_with_isometry(&self, isometry: nalgebra::Isometry2<FScalar>) -> Self {
    let mut new = self.clone();
    new.isometry = isometry;

    new
  }

  pub fn clone_with_pa(&self, pos: Vec3f, angle: Angle) -> Self {
    self.clone_with_isometry(PhysicalObject::create_isometry(pos, angle))
  }

  pub fn create_isometry(pos: Vec3f, angle: Angle) -> nalgebra::Isometry2<FScalar> {
    nalgebra::Isometry2::<FScalar>::new(vec_to_navec(pos), angle)
  }

  pub fn get_isometry(base: &entity::EntityBase) -> nalgebra::Isometry2<FScalar> {
    // TODO: not base.angle, but base.visible_angle
    // for example, a vehicle could be rotated at 15 degrees, but displayed as 0 degrees
    PhysicalObject::create_isometry(base.pos, base.angle)
  }

  pub fn update_isometry(&mut self, base: &entity::EntityBase) {
    // TODO: only update if needed
    self.isometry = PhysicalObject::get_isometry(base);
  }

  pub fn point_inside(&self, point: Vec3f) -> bool {
    self.nashape.contains_point(&self.isometry, &vec_to_napoint(point))
  }

  pub fn pos(&self) -> Vec3f {
    navec_to_vec(self.isometry.translation.vector)
  }

  pub fn angle(&self) -> Angle {
    self.isometry.rotation.angle()
  }

  pub fn get_response_vector(&self, other: &PhysicalObject) -> Option<Vec3f> {
    let contact = ncollide2d::query::contact(&self.isometry, &(*self.nashape),
                                             &other.isometry, &(*other.nashape),
                                             0.0);

    if let Some(contact) = contact {
      Some(navec_to_vec(contact.normal.into_inner() * contact.depth))
    } else {
      None
    }
  }

  pub fn collides_with(&self, other: &PhysicalObject) -> bool {
    let proximity = ncollide2d::query::proximity(&self.isometry, &(*self.nashape),
                                                 &other.isometry, &(*other.nashape),
                                                 0.0);

    proximity == ncollide2d::query::Proximity::Intersecting
  }
}
