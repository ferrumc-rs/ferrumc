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
                let mut rw_txn = env.write_txn()?;
                match env.create_database(&mut rw_txn, Some("players")) {
                    Ok(db) => {
                        rw_txn.commit()?;
                        db
                    }
                    Err(heed::Error::Io(_)) => {
                        let db = env.create_database(&mut rw_txn, Some("players"))?;
                        rw_txn.commit()?;
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

    pub fn save<S: Into<String>>(
        &self,
        key: S,
        pdc: &PersistentDataContainer,
    ) -> Result<(), PersistentDataError> {
        let encoded = bitcode::encode(&pdc.data);
        let mut wtxn = self.env.write_txn()?;

        if let Err(e) = self.db.put(&mut wtxn, key.into().as_str(), &encoded) {
            error!("Failed to add data to database: {}", e)
        }
        wtxn.commit()?;

        Ok(())
    }

    pub fn load<S: Into<String>>(
        &self,
        key: S,
    ) -> Result<PersistentDataContainer, PersistentDataError> {
        let rtxn = self
            .env
            .read_txn()
            .map_err(|_| PersistentDataError::FailedToReadDatabase)?;

        match self.db.get(&rtxn, key.into().as_str()) {
            Ok(Some(bytes)) => {
                let decoded: HashMap<String, Vec<u8>> = bitcode::decode(bytes)
                    .map_err(|_| PersistentDataError::DeserializationError)?;
                Ok(PersistentDataContainer {
                    data: decoded,
                    ..Default::default()
                })
            }
            _ => Ok(PersistentDataContainer::default()),
        }
    }
}
