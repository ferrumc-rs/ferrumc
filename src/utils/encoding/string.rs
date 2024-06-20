use std::io::{Read, Seek, Write};

use byteorder::{ReadBytesExt, WriteBytesExt};

use crate::utils;

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::utils::encoding::string::{read_string, write_string};

    #[test]
    fn read_string_valid_input() {
        let mut data = b"Test".to_vec();
        data.insert(0, 4);
        let mut cursor = Cursor::new(data);
        println!("{:?}", cursor.clone().into_inner());
        let result = read_string(&mut cursor);
        assert_eq!(result.unwrap(), "Test");
    }

    #[test]
    fn read_string_empty_input() {
        let mut cursor = Cursor::new(vec![]);
        let result = read_string(&mut cursor);
        assert!(result.is_err());
    }

    #[test]
    fn write_string_valid_input() {
        let mut cursor = Cursor::new(Vec::new());
        let result = write_string("Test", &mut cursor);
        assert!(result.is_ok());
        assert_eq!(cursor.into_inner(), vec![4, 0b01010100, 0b01100101, 0b01110011, 0b01110100]);
    }

    #[test]
    fn write_string_empty_input() {
        let mut cursor = Cursor::new(Vec::new());
        let result = write_string("", &mut cursor);
        assert!(result.is_ok());
        assert_eq!(cursor.into_inner(), vec![0b00000000]);
    }
}

pub fn read_string<T>(cursor: &mut T) -> crate::prelude::Result<String>
where T: Read + Unpin
{
    let length = utils::encoding::varint::read_varint(cursor)?;
    let mut buffer = vec![0u8; length as usize];
    for i in 0..length {
        buffer[i as usize] = cursor.read_u8()?;
    }
    Ok(String::from_utf8(buffer)?)
}

pub fn write_string<T>(string: &str, cursor: &mut T) -> crate::prelude::Result<()>
where T: Write  + Unpin
{
    let length = string.len();
    utils::encoding::varint::write_varint(length as i32, cursor)?;
    cursor.write_all(string.as_bytes())?;
    Ok(())
}