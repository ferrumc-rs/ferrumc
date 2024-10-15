mod errors;

use std::io::Read;
use std::path::PathBuf;
use memmap2::Mmap;
use tracing::error;
use crate::errors::AnvilError;

pub struct LoadedAnvilFile {
    pub table: [u8; 4096],
    data_map: Mmap,
}

pub fn get_chunk(x: u32, z: u32, file_path: PathBuf) -> Option<Vec<u8>> {
    let loaded_file = load_anvil_file(file_path).ok()?;
    loaded_file.get_chunk(x, z)
}

#[allow(unsafe_code)]
pub fn load_anvil_file(file_path: PathBuf) -> Result<LoadedAnvilFile, AnvilError> {

    // Check if the file exists
    if !file_path.exists() {
        return Err(AnvilError::FileNotFound(file_path));
    }

    match file_path.metadata() {
        Ok(meta) => {
            // We should have at least 8KB of data; 4KB for locations and 4KB for timestamps
            if meta.len() <= (4 * 1024) * 2 {
                return Err(AnvilError::InvalidTables(file_path));
            }
        }
        Err(e) => {
            return Err(AnvilError::UnableToReadFile(file_path, e));
        }
    }

    let file = std::fs::File::open(&file_path).map_err(
        |e| AnvilError::UnableToReadFile(file_path.clone(), e)
    )?;

    let res = unsafe { Mmap::map(&file) }.map_err(
        |e| AnvilError::UnableToMapFile(file_path, e)
    )?;

    let table = {
        let mut table = [0; 4096];
        table.copy_from_slice(&res[0..4096]);
        table
    };

    Ok(LoadedAnvilFile {
        table,
        data_map: res,
    })
}


impl LoadedAnvilFile {
    /// Get all the locations from the table
    /// Useful for finding all the chunks in the file, since you can then use `get_chunk_from_location` to get the chunk data
    /// Note for using rayon, chunks of 96 seems to be the best for performance

    #[allow(unsafe_code)]
    pub fn get_locations(&self) -> Vec<u32> {
        (0..1024).map(|i| {
            u32::from(self.table[i * 4]) << 24
                | u32::from(self.table[i * 4 + 1]) << 16
                | u32::from(self.table[i * 4 + 2]) << 8
                | u32::from(self.table[i * 4 + 3])
        })
            .filter(|&x| x != 0)
            .collect::<Vec<u32>>()
    }

    /// Get the data from the mmaped file, given an offset and size
    #[allow(unsafe_code)]
    fn get_data_from_file(&self, offset: u32, size: u32) -> Result<Vec<u8>, AnvilError> {
        unsafe {
            let start = self.data_map.as_ptr().add(offset as usize);
            let slice = std::slice::from_raw_parts(start, size as usize);
            Ok(slice.to_vec())
        }
    }

    /// Get the chunk data from a location
    ///
    /// The location is a 32-bit integer, where the first 24 bits are the offset in the file, and the last 8 bits are the size of the chunk
    ///
    /// The chunk data is compressed, and the first byte of the chunk data is the compression type
    ///
    /// The compression types are:
    ///
    /// 1: Gzip
    /// 2: Zlib
    /// 3: None
    /// 4: LZ4
    ///
    /// The next 4 bytes are the uncompressed size of the chunk
    ///
    /// The rest of the data is the compressed chunk data
    ///
    /// This function will return the decompressed chunk data
    ///
    /// If the compression type is unknown, it will return None
    ///
    /// If the decompression fails, it will return None
    ///
    /// If the location is invalid, it will return None
    pub fn get_chunk_from_location(&self, location: u32) -> Option<Vec<u8>> {
        let offset = ((location >> 8) & 0xFFFFFF) * 4096;
        let size = (location & 0xFF) * 4096;
        let chunk_data = self.get_data_from_file(offset, size).ok()?;
        let chunk_header = chunk_data[0..4].to_vec();
        let chunk_compressed_data = chunk_data[5..].to_vec();
        let uncompressed_size = u32::from(chunk_header[0]) << 24
            | u32::from(chunk_header[1]) << 16
            | u32::from(chunk_header[2]) << 8
            | u32::from(chunk_header[3]);
        let compression_type = chunk_data[4];
        match compression_type {
            1 => {
                let mut decompressed_data = Vec::new();
                let mut decoder = flate2::read::GzDecoder::new(&chunk_compressed_data[..]);
                decoder.read_to_end(&mut decompressed_data).unwrap();
                Some(decompressed_data)
            }
            2 => {
                let out = yazi::decompress(&chunk_compressed_data[..], yazi::Format::Zlib).ok();
                match out {
                    Some(data) => Some(data.0),
                    None => {
                        error!("Failed to decompress Zlib data");
                        None
                    }
                }
            }
            3 => Some(chunk_compressed_data),
            4 => {
                let mut decompressed_data = vec![0; uncompressed_size as usize];
                lzzzz::lz4::decompress(&chunk_compressed_data[..], &mut decompressed_data).unwrap();
                Some(decompressed_data)
            }
            _ => {
                error!("Unknown compression type: {}", compression_type);
                None
            }
        }
    }

    /// Get the chunk data from the table
    ///
    /// The x and z coordinates are the chunk coordinates
    ///
    /// This function will return the decompressed chunk data
    pub fn get_chunk(&self, x: u32, z: u32) -> Option<Vec<u8>> {
        let index = u64::from(4 * ((x % 32) + (z % 32) * 32));
        let location = u32::from(self.table[index as usize * 4]) << 24
            | u32::from(self.table[index as usize * 4 + 1]) << 16
            | u32::from(self.table[index as usize * 4 + 2]) << 8
            | u32::from(self.table[index as usize * 4 + 3]);
        self.get_chunk_from_location(location)
    }
}


#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Read;
    use fastanvil::Region;
    use super::*;
    use ferrumc_utils::root;
    use rayon::prelude::*;

    #[test]
    fn test_load_anvil_file() {
        let file_path = PathBuf::from(root!(".etc/r.0.0.mca"));
        let result = load_anvil_file(file_path);
        assert!(result.is_ok());
        let loaded_file = result.unwrap();
        let mut file = File::open(root!(".etc/r.0.0.mca")).unwrap();
        let mut buf: [u8; 4096] = [0; 4096];
        file.read_exact(&mut buf).unwrap();
        assert_eq!(loaded_file.table, buf);
    }

    #[test]
    fn test_get_chunk() {
        let file_path = PathBuf::from(root!(".etc/r.0.0.mca"));
        let loaded_file = load_anvil_file(file_path).unwrap();
        let chunk = loaded_file.get_chunk(0, 0);
        let fast_chunk = Region::from_stream(File::open(root!(".etc/r.0.0.mca")).unwrap()).unwrap().read_chunk(0, 0).unwrap();
        assert!(chunk.is_some());
        assert!(fast_chunk.is_some());
        assert_eq!(chunk.clone().unwrap(), fast_chunk.unwrap());
    }

    #[test]
    fn test_get_chunk_from_location() {
        let file_path = PathBuf::from(root!(".etc/r.0.0.mca"));
        let loaded_file = load_anvil_file(file_path).unwrap();
        let locations = loaded_file.get_locations();
        locations.chunks(96).par_bridge().for_each(|chunk| {
            chunk.iter().for_each(|location| {
                let _ = loaded_file.get_chunk_from_location(*location);
            });
        });
    }
}