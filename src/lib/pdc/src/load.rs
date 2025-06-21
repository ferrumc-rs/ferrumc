use crate::{container::PersistentDataContainer, errors::PersistentDataError};

pub struct PersistentContainerLoader;

impl PersistentContainerLoader {
    pub fn load_from_file(path: &str) -> Result<PersistentDataContainer, PersistentDataError> {
        todo!()
    }
}
