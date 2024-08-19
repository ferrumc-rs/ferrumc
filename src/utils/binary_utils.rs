use byteorder::ReadBytesExt;
use tokio::io::AsyncReadExt;

pub fn read_n_byte_integer<T>(data: T, n: usize) -> i32
where
    T: Into<Vec<u8>>,
{
    let data: Vec<u8> = data.into();
    let mut result = 0;
    for i in 0..n {
        result |= (data[i] as i32) << (8 * (n - i - 1));
    }
    result
}

pub fn read_n_byte_unsigned_integer<T>(data: T, n: usize) -> u32
where
    T: Into<Vec<u8>>,
{
    let data: Vec<u8> = data.into();
    let mut result = 0;
    for i in 0..n {
        result |= (data[i] as u32) << (8 * (n - i - 1));
    }
    result
}

pub fn read_n_byte_integer_stream<T>(data: &mut T, n: usize) -> i32
where
    T: std::io::Read,
{
    let mut result = 0;
    for i in 0..n {
        result |= (data.read_u8().unwrap() as i32) << (8 * (n - i - 1));
    }
    result
}
pub async fn read_n_byte_integer_stream_async<T>(data: &mut T, n: usize) -> i32
where
    T: tokio::io::AsyncRead + Unpin,
{
    let mut result = 0;
    for i in 0..n {
        result |= (data.read_u8().await.unwrap() as i32) << (8 * (n - i - 1));
    }
    result
}
