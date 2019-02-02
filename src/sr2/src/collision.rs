use super::*;
use ncollide2d::query::PointQuery;
use std::sync::Arc;

pub enum CollisionShape {
  Rect(Vec3i),
  Circle(IScalar)
}

// TODO: weight
pub struct EntityCollision {
  pub shape: Arc<ncollide2d::shape::Shape<FScalar>>,
  pub isometry: nalgebra::Isometry2<FScalar>,
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

impl EntityCollision {
  pub fn new_from_shape(shape: CollisionShape) -> Self {
    let shape: Arc<ncollide2d::shape::Shape<FScalar>> = match shape {
      CollisionShape::Rect(vec) => {
        Arc::new(ncollide2d::shape::Cuboid::new(vec_to_navec(vec.into())))
      },
      CollisionShape::Circle(radius) => {
        Arc::new(ncollide2d::shape::Ball::new(radius.into()))
      }
    };

    EntityCollision {
      shape,
      isometry: nalgebra::Isometry2::<FScalar>::identity()
    }
  }

  pub fn get_isometry(base: &entity::EntityBase) -> nalgebra::Isometry2<FScalar> {
    // TODO: not base.angle, but base.visible_angle
    // for example, a vehicle could be rotated at 15 degrees, but displayed as 0 degrees
    nalgebra::Isometry2::<FScalar>::new(vec_to_navec(base.pos), base.angle)
  }

  pub fn update_isometry(&mut self, base: &entity::EntityBase) {
    // TODO: only update if needed
    self.isometry = EntityCollision::get_isometry(base);
  }

  pub fn point_inside(&self, point: Vec3f) -> bool {
    self.shape.contains_point(&self.isometry, &vec_to_napoint(point))
  }

  pub fn get_response_vector(&self, other: &EntityCollision) -> Option<Vec3f> {
    let contact = ncollide2d::query::contact(&self.isometry, &(*self.shape),
                                             &other.isometry, &(*other.shape),
                                             0.0);

    if let Some(contact) = contact {
      Some(navec_to_vec(contact.normal.into_inner() * contact.depth))
    } else {
      None
    }
  }

  pub fn collides_with(&self, other: &EntityCollision) -> bool {
    let proximity = ncollide2d::query::proximity(&self.isometry, &(*self.shape),
                                                 &other.isometry, &(*other.shape),
                                                 0.0);

    proximity == ncollide2d::query::Proximity::Intersecting
  }
}
