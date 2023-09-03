use std::io::{Read, Write};
use anyhow::Error;
use anyhow::Result;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use crate::network::packet::Packet;


pub fn read_varlong<R: Read>(reader: &mut R) -> Result<i64> {
    // works fine
    reader.read_i64::<BigEndian>().map_err(Error::from)
}

// Works fine
pub async fn write_varlong<W: Write>(writer: &mut W, value: i64) -> Result<()> {
    writer.write_i64::<BigEndian>(value).map_err(Error::from)
}


// used in read_varint implemenetation
const LAST_SEVEN_BITS: i32 = 0b0111_1111;
const NEXT_BYTE_EXISTS: u8 = 0b1000_0000;

// bit mask to remove remaining 7  MSB's after right shift
const SEVEN_BITS_SHIFT_MASK: i32 = 0x01_ff_ff_ff;

pub(crate) async fn write_varint(sink: &mut Vec<u8>, mut value: i32) {
    loop {
        let mut temp = (value & LAST_SEVEN_BITS) as u8;
        // i32 right shift is arithmetic shift (preserves MSB)
        value >>= 7;
        value &= SEVEN_BITS_SHIFT_MASK;
        if value != 0 {
            temp |= NEXT_BYTE_EXISTS;
        }
        sink.push(temp);
        if value == 0 {
            break;
        }
    }
}

pub fn read_varint<R: Read>(mut reader: R) -> Result<i32> {
    let mut num_read = 0;
    let mut result = 0;
    let mut read = 0x80; // Dummy value to start the loop

    while (read & 0x80) != 0 {
        read = reader.read_u8()?; // Read one byte
        let val = read & 0x7F; // Take the last 7 bits of the byte
        result |= (val as i32) << (7 * num_read); // Shift the 7 bits to their proper place

        num_read += 1;

        if num_read > 5 {
            return Err(Error::msg("VarInt is too large"));
        }
    }

    Ok(result)
}


/**
    * Needs fixing.
    * Serialize a packet with a string.
    * The string is serialized as a VarInt containing the length of the string, followed by the actual string data.
 */
pub fn serialize_packet_with_str(packet: Box<dyn Packet>, string: String) -> Vec<u8> {
    let packet_id = packet.get_id();

    // Step 1: Create a temporary buffer containing the packet_id and the string data.
    let mut temp_buffer = vec![];
    temp_buffer.write_u32::<BigEndian>(packet_id).unwrap(); // Write packet_id
    temp_buffer.write_u32::<BigEndian>(string.len() as u32).unwrap(); // Write string length
    temp_buffer.extend_from_slice(string.as_bytes()); // Write string data

    // Step 2: Create the final buffer containing the length of the temp buffer and the actual temp buffer data.
    let mut final_buffer = vec![];
    final_buffer.write_u32::<BigEndian>(temp_buffer.len() as u32).unwrap(); // Write length of the packet
    final_buffer.extend_from_slice(&temp_buffer); // Write packet_id, string length, and string data

    final_buffer
}

