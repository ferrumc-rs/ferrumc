pub mod errors;

use std::io::Read;
use std::path::PathBuf;
use memmap2::Mmap;
use tracing::error;
use yazi::Adler32;
use crate::errors::AnvilError;

pub struct LoadedAnvilFile {
    pub table: [u8; 4096],
    data_map: Mmap,
}

pub fn get_chunk(x: u32, z: u32, file_path: PathBuf) -> Option<Vec<u8>> {
    let loaded_file = load_anvil_file(file_path).ok()?;
    loaded_file.get_chunk(x, z)
}

/// Memory map the file and return a `LoadedAnvilFile` struct
///
/// The `LoadedAnvilFile` struct contains the table and the data map and can be used to get chunk data
///
/// This is pretty fragile when it comes to things like other programs writing to the file while it's open
/// so be careful when using this and make sure to handle errors gracefully
///
/// Arguments:
///
/// * `file_path` - The path to the file
///
/// Returns:
///
/// * `Result<LoadedAnvilFile, AnvilError>` - The loaded anvil file
///
/// # Examples
///
/// ```no_run
/// use std::fs::File;
/// use fastanvil::Region;
/// use std::path::PathBuf;
/// use ferrumc_anvil::load_anvil_file;
///
/// let file_path = PathBuf::from("r.0.0.mca");
///
/// let mut fast_file = Region::from_stream(File::open(file_path.clone()).unwrap()).unwrap();
/// let loaded_file = load_anvil_file(file_path).unwrap();
///
/// let chunk = loaded_file.get_chunk(0, 0);
/// let fast_chunk = fast_file.read_chunk(0, 0).unwrap();
///
/// assert_eq!(chunk, fast_chunk);
/// ```
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
    ///
    /// The locations are 32-bit integers, where the first 24 bits are the offset in the file, and 
    /// the last 8 bits are the size of the chunk. Generally these aren't useful on their own, but
    /// can be used to get the chunk data with `get_chunk_from_location`. They are probably in order
    /// but not guaranteed to be
    pub fn get_locations(&self) -> Vec<u32> {
        let mut locations = Vec::with_capacity(1024);
        for i in 0..1024 {
            let location = u32::from(self.table[i * 4]) << 24
                | u32::from(self.table[i * 4 + 1]) << 16
                | u32::from(self.table[i * 4 + 2]) << 8
                | u32::from(self.table[i * 4 + 3]);
            if location != 0 {
                locations.push(location);
            }
        }
        locations
    }

    /// Get the data from the mmaped file, given an offset and size
    fn get_data_from_file(&self, offset: u32, size: u32) -> Result<&[u8], AnvilError> {
        let offset = offset as usize;
        let size = size as usize;

        // Early return if the requested range is out of bounds
        if offset + size > self.data_map.len() {
            return Err(AnvilError::InvalidOffsetOrSize);
        }

        // Return a reference to the slice (no allocation needed)
        Ok(&self.data_map[offset..offset + size])
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
        let offset = (location >> 8) & 0xFFFFFF;
        if u64::from(offset) * 4096 >= u64::from(u32::MAX) {
            error!("Invalid offset: {}", offset);
            return None;
        }
        let offset = offset * 4096;
        let size = (location & 0xFF) * 4096;
        let chunk_data = self.get_data_from_file(offset, size).ok()?;
        let chunk_compressed_data = &chunk_data[5..]; // No need to clone, just use the slice
        let compression_type = chunk_data[4];

        match compression_type {
            1 => {
                let mut decompressed_data = Vec::new();
                let mut decoder = flate2::read::GzDecoder::new(chunk_compressed_data);
                decoder.read_to_end(&mut decompressed_data).unwrap();
                Some(decompressed_data)
            }
            2 => {
                let out = yazi::decompress(chunk_compressed_data, yazi::Format::Zlib).ok();
                match out {
                    Some(data) => {
                        match data.1 {
                            Some(checksum) => {
                                if Adler32::from_buf(&data.0).finish() == checksum {
                                    Some(data.0)
                                } else {
                                    error!("Checksum does not match");
                                    None
                                }
                            }
                            None => {
                                error!("Failed to decompress Zlib data (No checksum)");
                                None
                            }
                        }
                    }
                    None => {
                        error!("Failed to decompress Zlib data");
                        None
                    }
                }
            }
            3 => Some(chunk_compressed_data.to_vec()),
            4 => {
                let mut decompressed_data = vec![];
                lzzzz::lz4::decompress(chunk_compressed_data, &mut decompressed_data).ok()?;
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
        let index = u64::from(4 * ((x & 31) + (z & 31) * 32));
        let base_index = index as usize * 4;
        let chunk_data = [
            u32::from(self.table[base_index]),
            u32::from(self.table[base_index + 1]),
            u32::from(self.table[base_index + 2]),
            u32::from(self.table[base_index + 3]),
        ];
        let location = (chunk_data[0] << 24) | (chunk_data[1] << 16) | (chunk_data[2] << 8) | chunk_data[3];
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
    fn test_bad_load_fails() {
        let file_path = PathBuf::from(root!(".etc/should_not_exist.mca"));
        let result = load_anvil_file(file_path);
        assert!(result.is_err());
    }

    #[test]
    fn invalid_read_fails() {
        let file_path = PathBuf::from(root!(".etc/codec.nbt"));
        let loaded_file = load_anvil_file(file_path).unwrap();
        let chunk = loaded_file.get_chunk(15, 3);
        assert!(chunk.is_none());
    }

    #[test]
    fn test_get_chunk() {
        ferrumc_logging::init_logging();
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