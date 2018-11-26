use super::*;

pub struct Game {
  pub name: &'static str,

  pub main: fn(archive: Box<Archive>, args: Vec<String>),
  pub check: fn(archive: &Archive) -> std::io::Result<bool>
}
