use std::fmt::Display;

use ferrumc_macros::Component;

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

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::utils::type_impls::{Decode, Encode};

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
