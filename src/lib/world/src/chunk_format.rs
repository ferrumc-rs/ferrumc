use bitcode_derive::{Decode, Encode};

#[derive(Encode, Decode)]
// This is a placeholder for the actual chunk format
pub struct Chunk {
    pub x: i32,
    pub z: i32,
}