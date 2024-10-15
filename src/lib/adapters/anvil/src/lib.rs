mod errors;

use std::path::PathBuf;
use memmap2::Mmap;
use crate::errors::AnvilError;

pub struct LoadedAnvilFile {
    pub table: [u8; 4096],
    current_index: u16,
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

    let table = unsafe {std::slice::from_raw_parts(res.as_ptr(), 4096).try_into().unwrap()};

    Ok(LoadedAnvilFile {
        table,
        current_index: 0,
    })
}


#[cfg(test)]
mod tests {
    use std::io::Read;
    use super::*;
    use ferrumc_utils::root;

    #[test]
    fn test_load_anvil_file() {
        let file_path = PathBuf::from(root!(".etc/r.0.0.mca"));
        let result = load_anvil_file(file_path);
        assert!(result.is_ok());
        let loaded_file = result.unwrap();
        let mut file = std::fs::File::open(root!(".etc/r.0.0.mca")).unwrap();
        let mut buf: [u8; 4096] = [0; 4096];
        file.read_exact(&mut buf).unwrap();
        assert_eq!(loaded_file.table, buf);
    }
}