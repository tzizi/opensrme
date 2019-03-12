use std::io::prelude::*;
use super::*;

pub trait Archive {
  fn new(filename: &str) -> Self where Self: Sized;
  fn open_file(&self, filename: &str) -> std::io::Result<std::io::Cursor<Vec<u8>>>;
  fn list_dir(&self, filename: &str) -> std::io::Result<Vec<String>>;
}



pub struct FilesystemArchive {
  root: String
}

fn remove_leading_slash(path: &str) -> String {
  String::from(String::from(path).trim_start_matches('/'))
}

fn get_fullpath(path1: &str, path: &str) -> std::ffi::OsString {
  std::path::PathBuf::from(path1).join(remove_leading_slash(path)).into_os_string()
}


impl Archive for FilesystemArchive {
  fn new(filename: &str) -> Self {
    FilesystemArchive {
      root: String::from(filename)
    }
  }

  fn open_file(&self, filename: &str) -> std::io::Result<std::io::Cursor<Vec<u8>>> {
    let mut file = std::fs::File::open(get_fullpath(&self.root[..], filename))?;
    let mut contents = vec![];
    file.read_to_end(&mut contents)?;
    Ok(std::io::Cursor::new(contents))
  }

  fn list_dir(&self, filename: &str) -> std::io::Result<Vec<String>> {
    let entries = std::fs::read_dir(get_fullpath(&self.root[..], filename))?;
    let mut out:Vec<String> = vec![];
    for entry in entries {
      if let Ok(entry) = entry {
        out.push(entry.file_name().into_string().unwrap());
      }
    }
    Ok(out)
  }
}
