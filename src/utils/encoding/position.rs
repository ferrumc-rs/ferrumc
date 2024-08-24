use std::fmt::Display;
use ferrumc_codec::enc::Encode;
use ferrumc_macros::Component;
use tokio::io::{AsyncWrite, AsyncWriteExt};

/// Represents a position in the world
///
/// Check out the [Position::encode] and [Position::decode]
/// implementations for more information on how this struct is encoded and decoded
#[derive(Clone, Component, Debug)]
pub struct Position {
    // Encoded as a 26 bit int
    pub x: i32,
    // Encoded as a 26 bit int
    pub z: i32,
    // Encoded as a 12 bit int
    pub y: i16,
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Position {
    /// Creates a new position
    pub fn new(x: i32, y: i16, z: i32) -> Self {
        Position { x, y, z }
    }
}

impl Encode for Position {
    /// Encodes a Position into a byte stream. A Position is a 64-bit integer, where the 26 MSB
    /// are the x coordinate, the next 26 bits are the z coordinate, and the 12 LSB are
    /// the y coordinate. This method encodes the x, y, and z coordinates into a 64-bit integer,
    /// and then writes the integer to the byte stream. The x and z coordinates are masked to 26 bits,
    /// and the y coordinate is masked to 12 bits. Uses [ferrumc_utils::encoding::position::Position]
    /// to represent the Position.
    async fn encode<T>(&self, bytes: &mut T) -> Result<(), ferrumc_codec::CodecError>
    where
        T: AsyncWrite + Unpin,
    {
        let u64val: u64 = ((self.x as u64 & 0x3FFFFFF) << 38)
            | ((self.z as u64 & 0x3FFFFFF) << 12)
            | (self.y as u64 & 0xFFF);
        let u64bytes = u64val.to_be_bytes();
        bytes
            .write_all(&u64bytes)
            .await
            // .map_err(|_| Error::Generic("Failed to write Position".parse().unwrap()))
            .map_err(ferrumc_codec::CodecError::from_external_error)
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::utils::impls::type_impls::{Decode};

    use super::Position;

    #[tokio::test]
    async fn test_position_decode() {
        let mut data = Cursor::new(
            0b01000110000001110110001100_10110000010101101101001000_001100111111_i64.to_be_bytes(),
        );
        let position = Position::decode(&mut data).await.unwrap();
        assert_eq!(position.x, 18357644);
        assert_eq!(position.z, -20882616);
        assert_eq!(position.y, 831);
    }

    #[tokio::test]
    async fn test_position_encode() {
        let position = Position {
            x: 18357644,
            z: -20882616,
            y: 831,
        };
        let mut data = Cursor::new(Vec::new());
        position.encode(&mut data).await.unwrap();
        let data = data.into_inner();
        assert_eq!(
            data,
            0b01000110000001110110001100_10110000010101101101001000_001100111111_i64.to_be_bytes()
        );
    }
}
