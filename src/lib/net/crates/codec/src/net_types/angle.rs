use crate::encode::{NetEncode, NetEncodeOpts, NetEncodeResult};
use std::f64::consts::PI;
use std::io::Write;
use tokio::io::AsyncWriteExt;

/// Represents a rotation angle in steps of 1/256 of a full turn
/// Stored as a single byte (0-255)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NetAngle(pub u8);

impl NetAngle {
    /// Creates a new Angle from a byte value
    pub fn new(value: u8) -> Self {
        NetAngle(value)
    }

    /// Creates an Angle from degrees
    pub fn from_degrees(deg: f64) -> Self {
        // This ensures negative angles won't break the `as u8` cast
        let wrapped = deg.rem_euclid(360.0);
        let steps = (wrapped * 256.0 / 360.0).round() as u8;
        NetAngle(steps)
    }

    /// Creates an Angle from radians
    pub fn from_radians(radians: f64) -> Self {
        let normalized = radians % (2.0 * PI);
        let steps = (normalized * 256.0 / (2.0 * PI)).round() as u8;
        NetAngle(steps)
    }

    /// Converts the angle to degrees
    pub fn to_degrees(&self) -> f64 {
        (self.0 as f64) * 360.0 / 256.0
    }

    /// Converts the angle to radians
    pub fn to_radians(&self) -> f64 {
        (self.0 as f64) * 2.0 * PI / 256.0
    }

    /// Returns the raw byte value
    pub fn as_byte(&self) -> u8 {
        self.0
    }
}

impl From<u8> for NetAngle {
    fn from(value: u8) -> Self {
        NetAngle(value)
    }
}

impl From<NetAngle> for u8 {
    fn from(angle: NetAngle) -> Self {
        angle.0
    }
}

impl NetEncode for NetAngle {
    fn encode<W: Write>(&self, writer: &mut W, _: &NetEncodeOpts) -> NetEncodeResult<()> {
        writer.write_all(&[self.0])?;
        Ok(())
    }
    async fn encode_async<W: tokio::io::AsyncWrite + Unpin>(
        &self,
        writer: &mut W,
        _: &NetEncodeOpts,
    ) -> NetEncodeResult<()> {
        writer.write_all(&[self.0]).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_angle_conversions() {
        let angle = NetAngle::from_degrees(90.0);
        assert!((angle.to_degrees() - 90.0).abs() < f64::EPSILON);

        let angle = NetAngle::from_radians(PI / 2.0);
        assert!((angle.to_radians() - PI / 2.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_angle_wraparound() {
        let angle1 = NetAngle::from_degrees(370.0);
        let angle2 = NetAngle::from_degrees(10.0);
        assert_eq!(angle1, angle2);
    }
}
