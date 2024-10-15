mod errors;

use std::io::Read;
use std::path::PathBuf;
use memmap2::Mmap;
use tracing::error;
use crate::errors::AnvilError;

pub struct LoadedAnvilFile {
    pub table: [u8; 4096],
    pub current_index: usize,
    data_map: Mmap
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
        current_index: 0,
        data_map: res,
    })
}


impl LoadedAnvilFile {

    fn get_data_from_file(&self, offset: u32, size: u32) -> Result<Vec<u8>, AnvilError> {
        Ok(self.data_map[offset as usize..(offset + size) as usize].to_vec())
    }
    
    fn get_chunk_from_location(&self, location: u32) -> Option<Vec<u8>> {
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
                let mut decompressed_data = Vec::new();
                let mut decoder = flate2::read::ZlibDecoder::new(&chunk_compressed_data[..]);
                decoder.read_to_end(&mut decompressed_data).unwrap();
                Some(decompressed_data)
            }
            3 => Some(chunk_compressed_data),
            4 => {
                Some(lz4_flex::decompress(&chunk_compressed_data[..], uncompressed_size as usize).unwrap())
            }
            _ => {
                error!("Unknown compression type: {}", compression_type);
                None
            }
        }
    }

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
}