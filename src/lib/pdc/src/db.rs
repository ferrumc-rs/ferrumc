use std::{collections::HashMap, path::Path};

use bevy_ecs::resource::Resource;
use heed::{
    Database, Env, EnvOpenOptions,
    types::{Bytes, Str},
};
use tracing::error;

use crate::{container::PersistentDataContainer, errors::PersistentDataError};

#[derive(Resource)]
pub struct PdcDatabaseResource {
    pub database: PdcDatabase,
}

pub struct PdcDatabase {
    env: Env,
    db: Database<Str, Bytes>,
}

impl PdcDatabase {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, PersistentDataError> {
        unsafe {
            let env = EnvOpenOptions::new()
                .max_dbs(1)
                .open(path)
                .map_err(|_| PersistentDataError::FailedToOpenDatabase)?;

            let db: Database<Str, Bytes> = {
                let mut rw_txn = env.write_txn().unwrap();
                match env.create_database(&mut rw_txn, Some("players")) {
                    Ok(db) => {
                        rw_txn.commit().unwrap();
                        db
                    }
                    Err(heed::Error::Io(_)) => {
                        let db = env.create_database(&mut rw_txn, Some("players")).unwrap();
                        rw_txn.commit().unwrap();
                        db
                    }
                    Err(_) => return Err(PersistentDataError::FailedToOpenDatabase),
                }
            };

            Ok(PdcDatabase {
                env: env.clone(),
                db,
            })
        }
    }

    pub fn save(
        &self,
        key: &str,
        pdc: &PersistentDataContainer,
    ) -> Result<(), PersistentDataError> {
        let encoded = bitcode::encode(&pdc.data);
        let mut wtxn = self.env.write_txn().unwrap();

        if let Err(e) = self.db.put(&mut wtxn, key, &encoded) {
            error!("Failed to add data to database: {}", e)
        }
        wtxn.commit().unwrap();

        Ok(())
    }

    pub fn load(&self, key: &str) -> Result<PersistentDataContainer, PersistentDataError> {
        let rtxn = self
            .env
            .read_txn()
            .map_err(|_| PersistentDataError::FailedToReadDatabase)?;

        match self.db.get(&rtxn, key) {
            Ok(Some(bytes)) => {
                let decoded: HashMap<String, Vec<u8>> = bitcode::decode(bytes)
                    .map_err(|_| PersistentDataError::DeserializationError)?;
                Ok(PersistentDataContainer {
                    data: decoded,
                    ..Default::default()
                })
            }
            Ok(None) => Ok(PersistentDataContainer::default()),
            Err(_) => Ok(PersistentDataContainer::default()),
        }
    }
}
