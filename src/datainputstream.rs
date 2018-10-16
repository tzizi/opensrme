use std::io;
use std::io::Read;
/*use std::io::Read;
use std::io::Cursor;*/
use byteorder::{BigEndian, ReadBytesExt};

pub trait DataInputStream: ReadBytesExt {
  fn readShort(&mut self) -> io::Result<i16>;
  fn readInt(&mut self) -> io::Result<i32>;
  fn readUTF(self: &mut Self) -> io::Result<String>;
}

impl<T: Read> DataInputStream for T {
  fn readShort(&mut self) -> io::Result<i16> {
    self.read_i16::<BigEndian>()
  }

  fn readInt(&mut self) -> io::Result<i32> {
    self.read_i32::<BigEndian>()
  }

  fn readUTF(&mut self) -> io::Result<String> {
    let size = self.readShort()?;
    let mut string = String::from("");
    self.take(size as u64).read_to_string(&mut string)?;
    Ok(string)
  }
}
