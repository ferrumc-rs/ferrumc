use std::io::{Cursor, Read};
use crate::v2_parser::{NBTError, NBTResult};

pub trait CursorExt {
    fn read_i8(&mut self) -> NBTResult<i8>;
    fn read_i16(&mut self) -> NBTResult<i16>;
    fn read_i32(&mut self) -> NBTResult<i32>;
    fn read_i64(&mut self) -> NBTResult<i64>;
    fn read_f32(&mut self) -> NBTResult<f32>;
    fn read_f64(&mut self) -> NBTResult<f64>;
    fn read_nbt_string(&mut self) -> NBTResult<String>;
    fn read_string_with_len(&mut self, len: u16) -> NBTResult<String>;

    // Unsafe versions of the above functions
    // fast but unsafe
    /*unsafe fn read_i8_unchecked(&mut self) -> i8;
    unsafe fn read_i16_unchecked(&mut self) -> i16;
    unsafe fn read_i32_unchecked(&mut self) -> i32;
    unsafe fn read_i64_unchecked(&mut self) -> i64;
    unsafe fn read_f32_unchecked(&mut self) -> f32;
    unsafe fn read_f64_unchecked(&mut self) -> f64;
    unsafe fn read_nbt_string_unchecked(&mut self) -> String;
    unsafe fn read_string_with_len_unchecked(&mut self, len: u16) -> String;*/
}

impl CursorExt for Cursor<Vec<u8>> {
    #[inline]
    fn read_i8(&mut self) -> NBTResult<i8> {
        let mut buf = [0; 1];
        self.read_exact(&mut buf).map_err(|_| NBTError::ReadWriteError)?;
        Ok(i8::from_be_bytes(buf))
    }

    #[inline]
    fn read_i16(&mut self) -> NBTResult<i16> {
        let mut buf = [0; 2];
        self.read_exact(&mut buf).map_err(|_| NBTError::ReadWriteError)?;
        Ok(i16::from_be_bytes(buf))
    }

    #[inline]
    fn read_i32(&mut self) -> NBTResult<i32> {
        let mut buf = [0; 4];
        self.read_exact(&mut buf).map_err(|_| NBTError::ReadWriteError)?;
        Ok(i32::from_be_bytes(buf))
    }

    #[inline]
    fn read_i64(&mut self) -> NBTResult<i64> {
        let mut buf = [0; 8];
        self.read_exact(&mut buf).map_err(|_| NBTError::ReadWriteError)?;
        Ok(i64::from_be_bytes(buf))
    }

    #[inline]
    fn read_f32(&mut self) -> NBTResult<f32> {
        let mut buf = [0; 4];
        self.read_exact(&mut buf).map_err(|_| NBTError::ReadWriteError)?;
        Ok(f32::from_be_bytes(buf))
    }

    #[inline]
    fn read_f64(&mut self) -> NBTResult<f64> {
        let mut buf = [0; 8];
        self.read_exact(&mut buf).map_err(|_| NBTError::ReadWriteError)?;
        Ok(f64::from_be_bytes(buf))
    }

    #[inline]
    fn read_nbt_string(&mut self) -> NBTResult<String> {
        let len = self.read_i16()?;
        self.read_string_with_len(len as u16)
    }

    #[inline]
    fn read_string_with_len(&mut self, len: u16) -> NBTResult<String> {
        let mut buf = vec![0; len as usize];
        self.read_exact(&mut buf).map_err(|_| NBTError::ReadWriteError)?;
        // String::from_utf8_unchecked(buf).map_err(|e| NBTError::StringReadError(e.utf8_error()))
        unsafe {
            Ok(String::from_utf8_unchecked(buf))
        }
    }
/*
    #[inline(always)]
    unsafe fn read_i8_unchecked(&mut self) -> i8 {
        let pos = self.position() as usize;
        self.set_position(pos as u64 + 1);
        *self.get_ref().get_unchecked(pos) as i8
    }

    #[inline(always)]
    unsafe fn read_i16_unchecked(&mut self) -> i16 {
        let pos = self.position() as usize;
        self.set_position(pos as u64 + 2);
        i16::from_be_bytes(*(self.get_ref().as_ptr().add(pos) as *const [u8; 2]))
    }

    #[inline(always)]
    unsafe fn read_i32_unchecked(&mut self) -> i32 {
        let pos = self.position() as usize;
        self.set_position(pos as u64 + 4);
        i32::from_be_bytes(*(self.get_ref().as_ptr().add(pos) as *const [u8; 4]))
    }

    #[inline(always)]
    unsafe fn read_i64_unchecked(&mut self) -> i64 {
        let pos = self.position() as usize;
        self.set_position(pos as u64 + 8);
        i64::from_be_bytes(*(self.get_ref().as_ptr().add(pos) as *const [u8; 8]))
    }

    #[inline(always)]
    unsafe fn read_f32_unchecked(&mut self) -> f32 {
        let pos = self.position() as usize;
        self.set_position(pos as u64 + 4);
        f32::from_be_bytes(*(self.get_ref().as_ptr().add(pos) as *const [u8; 4]))
    }

    #[inline(always)]
    unsafe fn read_f64_unchecked(&mut self) -> f64 {
        let pos = self.position() as usize;
        self.set_position(pos as u64 + 8);
        f64::from_be_bytes(*(self.get_ref().as_ptr().add(pos) as *const [u8; 8]))
    }

    #[inline(always)]
    unsafe fn read_nbt_string_unchecked(&mut self) -> String {
        let len = self.read_i16_unchecked() as usize;
        self.read_string_with_len_unchecked(len as u16)
    }

    #[inline(always)]
    unsafe fn read_string_with_len_unchecked(&mut self, len: u16) -> String {
        let pos = self.position() as usize;
        self.set_position(pos as u64 + len as u64);
        let slice = std::slice::from_raw_parts(self.get_ref().as_ptr().add(pos), len as usize);
        String::from_utf8_unchecked(slice.to_vec())
    }*/
}
