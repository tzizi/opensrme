use super::*;

pub struct Game {
  pub name: &'static str,

  pub main: fn(archive: &Archive, args: Vec<String>),
  pub check: fn(archive: &Archive) -> std::io::Result<bool>
}
