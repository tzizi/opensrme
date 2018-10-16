use std::io;
use std::io::{Read};
use std::collections::*;
use regex::Regex;

#[derive(Debug)]
pub struct Manifest(pub HashMap<String, String>);


impl Manifest {
  pub fn new_from_file<T: Read>(file: &mut T) -> io::Result<Self> {
    let mut manifest = Manifest(HashMap::new());

    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    let re = Regex::new(r"^([^:]*): *(.*)$").unwrap();
    let whitespace_re = Regex::new(r"^\s*$").unwrap();

    for line in buffer.lines() {
      if whitespace_re.is_match(line) {
        continue;
      }

      let mat = re.captures(line);
      if let Some(mat) = mat {
        manifest.0.insert(String::from(mat.get(1).unwrap().as_str()),
                          String::from(mat.get(2).unwrap().as_str()));
      } else {
        println!("[warning] Invalid line: {}", line);
      }
    }

    Ok(manifest)
  }
}

#[macro_export]
macro_rules! check_manifest {
  ($manifest:ident, $($key:tt : $value:tt),*) => {{
    {
      let mut ok = true;

      $(
        if ok {
          ok = match $manifest.0.get($key) {
            Some(val) => if val != $value { false } else { true },
            None => false
          };
        }
      ),*;

      ok
    }
  }}
}
