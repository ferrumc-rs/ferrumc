use std::io;
use tokio::io::{AsyncWrite, AsyncWriteExt};

pub struct NBTBuffer {
    buffer: Vec<u8>,
}

impl NBTBuffer {
    pub fn new() -> Self {
        Self { buffer: Vec::with_capacity(1024) }
    }

    pub fn write_u8(&mut self, value: u8) {
        self.buffer.push(value);
    }
    pub fn write_i8(&mut self, value: i8) {
        self.buffer.extend_from_slice(&value.to_be_bytes());
    }
    pub fn write_bool(&mut self, value: bool) {
        self.write_u8(if value { 1 } else { 0 });
    }
    pub fn write_u16(&mut self, value: u16) {
        self.buffer.extend_from_slice(&value.to_be_bytes());
    }
    pub fn write_i16(&mut self, value: i16) {
        self.buffer.extend_from_slice(&value.to_be_bytes());
    }

    pub fn write_i32(&mut self, value: i32) {
        self.buffer.extend_from_slice(&value.to_be_bytes());
    }

    pub fn write_i64(&mut self, value: i64) {
        self.buffer.extend_from_slice(&value.to_be_bytes());
    }

    pub fn write_f32(&mut self, value: f32) {
        self.buffer.extend_from_slice(&value.to_be_bytes());
    }

    pub fn write_f64(&mut self, value: f64) {
        self.buffer.extend_from_slice(&value.to_be_bytes());
    }

    pub fn write_string(&mut self, value: &str) {
        self.write_u16(value.len() as u16);
        self.buffer.extend_from_slice(value.as_bytes());
    }

    pub async fn flush<W: AsyncWrite + Unpin + Send>(&mut self, writer: &mut W) -> io::Result<()> {
        writer.write_all(&self.buffer).await?;
        self.buffer.clear();
        Ok(())
    }
}