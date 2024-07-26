use std::{
    io::Cursor,
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use crate::error::UnexpectedEofError;

pub struct Reader<'a> {
    pub cur: *const u8,
    /// pointer to after the last byte (so remaining=end-cur)
    end: *const u8,

    _marker: PhantomData<&'a ()>,
}

impl<'a> Reader<'a> {
    pub fn new(data: &'a [u8]) -> Reader<'a> {
        Self {
            cur: data.as_ptr(),
            end: unsafe { data.as_ptr().add(data.len()) },
            _marker: PhantomData,
        }
    }

    pub fn ensure_can_read(&self, size: usize) -> Result<(), UnexpectedEofError> {
        let data_addr = self.cur as usize;
        let end_addr = self.end as usize;
        if data_addr + size > end_addr {
            Err(UnexpectedEofError)
        } else {
            Ok(())
        }
    }

    pub unsafe fn unchecked_read_type<T>(&mut self) -> T {
        let value = unsafe { self.cur.cast::<T>().read_unaligned() };
        self.cur = unsafe { self.cur.add(std::mem::size_of::<T>()) };
        value
    }

    pub fn read_type<T: Copy>(&mut self) -> Result<T, UnexpectedEofError> {
        self.ensure_can_read(std::mem::size_of::<T>())?;
        Ok(unsafe { self.unchecked_read_type() })
    }

    #[inline]
    pub fn read_u8(&mut self) -> Result<u8, UnexpectedEofError> {
        self.read_type()
    }
    #[inline]
    pub fn read_i8(&mut self) -> Result<i8, UnexpectedEofError> {
        self.read_u8().map(|x| x as i8)
    }

    #[inline]
    pub fn read_u16(&mut self) -> Result<u16, UnexpectedEofError> {
        let value = self.read_type::<u16>();
        #[cfg(target_endian = "little")]
        let value = value.map(u16::swap_bytes);
        value
    }
    #[inline]
    pub fn read_i16(&mut self) -> Result<i16, UnexpectedEofError> {
        self.read_u16().map(|x| x as i16)
    }

    #[inline]
    pub fn read_u32(&mut self) -> Result<u32, UnexpectedEofError> {
        let value = self.read_type::<u32>();
        #[cfg(target_endian = "little")]
        let value = value.map(u32::swap_bytes);
        value
    }
    #[inline]
    pub fn read_i32(&mut self) -> Result<i32, UnexpectedEofError> {
        self.read_u32().map(|x| x as i32)
    }

    #[inline]
    pub fn read_u64(&mut self) -> Result<u64, UnexpectedEofError> {
        let value = self.read_type::<u64>();
        #[cfg(target_endian = "little")]
        let value = value.map(u64::swap_bytes);
        value
    }
    #[inline]
    pub fn read_i64(&mut self) -> Result<i64, UnexpectedEofError> {
        self.read_u64().map(|x| x as i64)
    }

    #[inline]
    pub fn read_f32(&mut self) -> Result<f32, UnexpectedEofError> {
        self.read_u32().map(f32::from_bits)
    }

    #[inline]
    pub fn read_f64(&mut self) -> Result<f64, UnexpectedEofError> {
        self.read_u64().map(f64::from_bits)
    }

    #[inline]
    pub fn skip(&mut self, size: usize) -> Result<(), UnexpectedEofError> {
        self.ensure_can_read(size)?;
        self.cur = unsafe { self.cur.add(size) };
        Ok(())
    }

    #[inline]
    pub fn read_slice(&mut self, size: usize) -> Result<&'a [u8], UnexpectedEofError> {
        self.ensure_can_read(size)?;
        let slice = unsafe { std::slice::from_raw_parts(self.cur, size) };
        self.cur = unsafe { self.cur.add(size) };
        Ok(slice)
    }
}

impl<'a> From<&'a [u8]> for Reader<'a> {
    fn from(data: &'a [u8]) -> Self {
        Self::new(data)
    }
}

pub struct ReaderFromCursor<'a: 'cursor, 'cursor> {
    reader: Reader<'a>,
    original_cursor: &'cursor mut Cursor<&'a [u8]>,
}

impl<'a, 'cursor> ReaderFromCursor<'a, 'cursor> {
    pub fn new(cursor: &'cursor mut Cursor<&'a [u8]>) -> Self {
        let data = cursor.get_ref();
        Self {
            reader: Reader::new(data[cursor.position() as usize..].as_ref()),
            original_cursor: cursor,
        }
    }
}
impl Drop for ReaderFromCursor<'_, '_> {
    fn drop(&mut self) {
        self.original_cursor.set_position(
            (self.reader.cur as usize - self.original_cursor.get_ref().as_ptr() as usize) as u64,
        );
    }
}

impl<'a> Deref for ReaderFromCursor<'a, '_> {
    type Target = Reader<'a>;

    fn deref(&self) -> &Self::Target {
        &self.reader
    }
}
impl<'a> DerefMut for ReaderFromCursor<'a, '_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.reader
    }
}
