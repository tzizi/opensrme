use std::io;
use std::io::{Read, Seek};
use byteorder::{BigEndian, ReadBytesExt};

pub trait DataInputStream: ReadBytesExt {
  fn skip(&mut self, amount: usize) -> io::Result<()>;
  fn read_amount_as_u8(&mut self, amount: usize) -> io::Result<Vec<u8>>;
  fn read_amount(&mut self, amount: usize) -> io::Result<Vec<i8>>;
  fn read_amount_as_string(&mut self, amount: usize) -> io::Result<String>;
  fn read_byte(&mut self) -> io::Result<i8>;
  fn read_boolean(&mut self) -> io::Result<bool>;
  fn read_unsigned_byte(&mut self) -> io::Result<u8>;
  fn read_short(&mut self) -> io::Result<i16>;
  fn read_unsigned_short(&mut self) -> io::Result<u16>;
  fn readInt(&mut self) -> io::Result<i32>;
  fn read_utf(self: &mut Self) -> io::Result<String>;
}

impl<T: Read+Seek> DataInputStream for T {
  fn skip(&mut self, amount: usize) -> io::Result<()> {
    //self.take(amount as u64).read;
    try!(self.seek(std::io::SeekFrom::Current(amount as i64)));
    Ok(())
  }

  fn read_amount_as_u8(&mut self, amount: usize) -> io::Result<Vec<u8>> {
    let mut array: Vec<u8> = vec![];
    self.take(amount as u64).read_to_end(&mut array)?;

    Ok(array)
  }

  fn read_amount(&mut self, amount: usize) -> io::Result<Vec<i8>> {
    let array = self.read_amount_as_u8(amount)?;

    let mut array1: Vec<i8> = vec![];
    for value in array {
      array1.push(value as i8);
    }

    Ok(array1)
  }

  fn read_amount_as_string(&mut self, amount: usize) -> io::Result<String> {
    let mut string = String::from("");
    self.take(amount as u64).read_to_string(&mut string)?;
    Ok(string)
  }

  fn read_byte(&mut self) -> io::Result<i8> {
    Ok(self.read_unsigned_byte()? as i8)
  }

  fn read_boolean(&mut self) -> io::Result<bool> {
    Ok(self.read_unsigned_byte()? != 0)
  }

  fn read_unsigned_byte(&mut self) -> io::Result<u8> {
    let mut buf = [0; 1];
    try!(self.read_exact(&mut buf));
    Ok(buf[0])
  }

  fn read_short(&mut self) -> io::Result<i16> {
    self.read_i16::<BigEndian>()
  }

  fn read_unsigned_short(&mut self) -> io::Result<u16> {
    self.read_u16::<BigEndian>()
  }

  fn readInt(&mut self) -> io::Result<i32> {
    self.read_i32::<BigEndian>()
  }

  fn read_utf(&mut self) -> io::Result<String> {
    let size = self.read_short()?;
    /*let mut string = String::from("");
    self.take(size as u64).read_to_string(&mut string)?;
    Ok(string)*/
    self.read_amount_as_string(size as usize)
  }
}
