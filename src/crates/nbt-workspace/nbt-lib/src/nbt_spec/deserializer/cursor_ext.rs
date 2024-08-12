use std::io::{Cursor, Read};
use crate::error::NBTError;
use crate::NBTResult;

pub trait CursorExt {
    fn read_i8(&mut self) -> NBTResult<i8>;
    fn read_i16(&mut self) -> NBTResult<i16>;
    fn read_i32(&mut self) -> NBTResult<i32>;
    fn read_i64(&mut self) -> NBTResult<i64>;
    fn read_f32(&mut self) -> NBTResult<f32>;
    fn read_f64(&mut self) -> NBTResult<f64>;
    fn read_nbt_string(&mut self) -> NBTResult<String>;
    fn read_string_with_len(&mut self, len: u16) -> NBTResult<String>;
}

impl CursorExt for Cursor<Vec<u8>> {
    fn read_i8(&mut self) -> NBTResult<i8> {
        let mut buf = [0; 1];
        self.read_exact(&mut buf).map_err(NBTError::ReadWriteError)?;
        Ok(i8::from_be_bytes(buf))
    }

    fn read_i16(&mut self) -> NBTResult<i16> {
        let mut buf = [0; 2];
        self.read_exact(&mut buf).map_err(NBTError::ReadWriteError)?;
        Ok(i16::from_be_bytes(buf))
    }

    fn read_i32(&mut self) -> NBTResult<i32> {
        let mut buf = [0; 4];
        self.read_exact(&mut buf).map_err(NBTError::ReadWriteError)?;
        Ok(i32::from_be_bytes(buf))
    }

    fn read_i64(&mut self) -> NBTResult<i64> {
        let mut buf = [0; 8];
        self.read_exact(&mut buf).map_err(NBTError::ReadWriteError)?;
        Ok(i64::from_be_bytes(buf))
    }

    fn read_f32(&mut self) -> NBTResult<f32> {
        let mut buf = [0; 4];
        self.read_exact(&mut buf).map_err(NBTError::ReadWriteError)?;
        Ok(f32::from_be_bytes(buf))
    }

    fn read_f64(&mut self) -> NBTResult<f64> {
        let mut buf = [0; 8];
        self.read_exact(&mut buf).map_err(NBTError::ReadWriteError)?;
        Ok(f64::from_be_bytes(buf))
    }

    fn read_nbt_string(&mut self) -> NBTResult<String> {
        let len = self.read_i16()?;
        self.read_string_with_len(len as u16)
    }

    fn read_string_with_len(&mut self, len: u16) -> NBTResult<String> {
        let mut buf = vec![0; len as usize];
        self.read_exact(&mut buf).map_err(NBTError::ReadWriteError)?;
        String::from_utf8(buf).map_err(|e| NBTError::StringReadError(e.utf8_error()))
    }
}
