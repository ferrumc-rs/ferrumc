use rusqlite::{params, params_from_iter, Connection};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;
use std::{collections::HashMap, marker::PhantomData, path::PathBuf};

use crate::{database::Database, errors::StorageError};

// TODO: Implement proper error mapping
impl From<rusqlite::Error> for StorageError {
    fn from(err: rusqlite::Error) -> Self {
        StorageError::DatabaseError(err.to_string())
    }
}

#[derive(Debug, Clone)]
pub struct SqliteDatabase<T> {
    store_path: PathBuf,
    _marker: PhantomData<T>,
}

impl<T> SqliteDatabase<T> {
    pub fn initialize(
        store_path: Option<PathBuf>,
        storage_name: &str,
    ) -> Result<Self, StorageError> {
        let Some(checked_path) = store_path else {
            return Err(StorageError::InvalidPath);
        };
        if !checked_path.exists() {
            std::fs::create_dir_all(&checked_path)?;
        }
        let checked_path = checked_path.join(storage_name);
        Ok(Self {
            store_path: checked_path,
            _marker: PhantomData,
        })
    }

    fn open_conn(&self) -> Result<Connection, StorageError> {
        Ok(Connection::open(self.store_path.as_path())?)
    }
}

impl<T> Database for SqliteDatabase<T>
where
    T: Serialize + DeserializeOwned,
{
    type Key = u128;
    type Value = T;

    fn create_table(&self, table: &str) -> Result<(), StorageError> {
        let conn = self.open_conn()?;
        let sql = format!(
            "CREATE TABLE IF NOT EXISTS \"{}\" (key TEXT PRIMARY KEY, value JSON NOT NULL)",
            table
        );
        conn.execute(&sql, [])?;
        Ok(())
    }

    fn insert(&self, table: &str, key: Self::Key, value: Self::Value) -> Result<(), StorageError> {
        let conn = self.open_conn()?;
        let json_val: Value =
            serde_json::to_value(&value).map_err(|e| StorageError::DatabaseError(e.to_string()))?;
        let sql = format!("INSERT INTO \"{}\" (key, value) VALUES (?1, ?2)", table);
        conn.execute(&sql, params![key.to_string(), json_val])?;
        Ok(())
    }

    fn get(&self, table: &str, key: Self::Key) -> Result<Option<Self::Value>, StorageError> {
        let conn = self.open_conn()?;
        let sql = format!("SELECT value FROM \"{}\" WHERE key = ?1 LIMIT 1", table);
        let mut stmt = conn.prepare(&sql)?;
        let mut rows = stmt.query(params![key.to_string()])?;
        if let Some(row) = rows.next()? {
            let json_val: Value = row.get(0)?;
            let v: T = serde_json::from_value(json_val)
                .map_err(|e| StorageError::DatabaseError(e.to_string()))?;
            Ok(Some(v))
        } else {
            Ok(None)
        }
    }

    fn delete(&self, table: &str, key: Self::Key) -> Result<(), StorageError> {
        let conn = self.open_conn()?;
        let sql = format!("DELETE FROM \"{}\" WHERE key = ?1", table);
        conn.execute(&sql, params![key.to_string()])?;
        Ok(())
    }

    fn update(&self, table: &str, key: Self::Key, value: Self::Value) -> Result<(), StorageError> {
        let conn = self.open_conn()?;
        let json_val: Value =
            serde_json::to_value(&value).map_err(|e| StorageError::DatabaseError(e.to_string()))?;
        let sql = format!("UPDATE \"{}\" SET value = ?1 WHERE key = ?2", table);
        conn.execute(&sql, params![json_val, key.to_string()])?;
        Ok(())
    }

    fn upsert(
        &self,
        table: &str,
        key: Self::Key,
        value: &Self::Value,
    ) -> Result<bool, StorageError> {
        let conn = self.open_conn()?;
        let json_val: Value =
            serde_json::to_value(&value).map_err(|e| StorageError::DatabaseError(e.to_string()))?;
        let sql = format!(
            "INSERT INTO \"{t}\" (key, value) VALUES (?1, ?2)
             ON CONFLICT(key) DO UPDATE SET value = excluded.value",
            t = table
        );
        conn.execute(&sql, params![key.to_string(), json_val])?;
        Ok(true)
    }

    fn batch_insert(
        &self,
        table: &str,
        data: Vec<(Self::Key, Self::Value)>,
    ) -> Result<(), StorageError> {
        if data.is_empty() {
            return Ok(());
        }
        let mut conn = self.open_conn()?;
        let tx = conn.transaction()?;
        let sql = format!("INSERT INTO \"{}\" (key, value) VALUES (?1, ?2)", table);
        {
            let mut stmt = tx.prepare(&sql)?;
            for (k, v) in data {
                let json_val: Value = serde_json::to_value(&v)
                    .map_err(|e| StorageError::DatabaseError(e.to_string()))?;
                stmt.execute(params![k.to_string(), json_val])?;
            }
        }
        tx.commit()?;
        Ok(())
    }

    fn batch_get(
        &self,
        table: &str,
        keys: Vec<Self::Key>,
    ) -> Result<Vec<Option<Self::Value>>, StorageError> {
        if keys.is_empty() {
            return Ok(Vec::new());
        }
        let conn = self.open_conn()?;
        let placeholders = std::iter::repeat_n("?", keys.len())
            .collect::<Vec<_>>()
            .join(",");
        let query_sql = format!(
            "SELECT key, value FROM \"{}\" WHERE key IN ({})",
            table, placeholders
        );
        let mut stmt = conn.prepare(&query_sql)?;
        let key_strings = keys.iter().map(|k| k.to_string());
        let mut rows = stmt.query(params_from_iter(key_strings))?;

        let mut found: HashMap<String, serde_json::Value> = HashMap::new();
        while let Some(row) = rows.next()? {
            let key_str: String = row.get(0)?;
            let json_val: serde_json::Value = row.get(1)?;
            found.insert(key_str, json_val);
        }

        let mut result = Vec::with_capacity(keys.len());
        for k in keys {
            let ks = k.to_string();
            if let Some(json_val) = found.remove(&ks) {
                let v: T = serde_json::from_value(json_val)
                    .map_err(|e| StorageError::DatabaseError(e.to_string()))?;
                result.push(Some(v));
            } else {
                result.push(None);
            }
        }

        Ok(result)
    }

    fn batch_upsert(
        &self,
        table: &str,
        data: Vec<(Self::Key, Self::Value)>,
    ) -> Result<(), StorageError> {
        if data.is_empty() {
            return Ok(());
        }
        let mut conn = self.open_conn()?;
        let tx = conn.transaction()?;
        let sql = format!(
            "INSERT INTO \"{t}\" (key, value) VALUES (?1, ?2)
             ON CONFLICT(key) DO UPDATE SET value = excluded.value",
            t = table
        );
        {
            let mut stmt = tx.prepare(&sql)?;
            for (k, v) in data {
                let json_val: Value = serde_json::to_value(&v)
                    .map_err(|e| StorageError::DatabaseError(e.to_string()))?;
                stmt.execute(params![k.to_string(), json_val])?;
            }
        }
        tx.commit()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::fs::remove_dir_all;

    use super::*;
    use serde::Deserialize;
    use tempfile::tempdir;

    #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
    struct TestData {
        pub pos: Position,
        pub dimension: String,
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
    pub struct Position {
        pub x: f64,
        pub y: f64,
        pub z: f64,
    }

    fn setup_db() -> (SqliteDatabase<TestData>, String, tempfile::TempDir) {
        let dir = tempdir().unwrap();
        let db_path = dir.path();
        let db: SqliteDatabase<TestData> =
            SqliteDatabase::initialize(Some(PathBuf::from(db_path)), "test.db").unwrap();
        let table = "test_table".to_string();
        db.create_table(&table).unwrap();
        (db, table, dir)
    }

    #[test]
    fn test_insert_and_get() {
        let (db, table, db_path) = setup_db();
        let pos = Position {
            x: 0.0,
            y: 64.0,
            z: 0.0,
        };

        let data1 = TestData {
            dimension: "Nether".into(),
            pos: pos.clone(),
        };
        let data2 = TestData {
            dimension: "Overworld".into(),
            pos: pos.clone(),
        };

        db.insert(&table, 1001, data1.clone()).unwrap();
        db.insert(&table, 1002, data2.clone()).unwrap();

        assert_eq!(db.get(&table, 1001).unwrap(), Some(data1));
        assert_eq!(db.get(&table, 1002).unwrap(), Some(data2));
        assert_eq!(db.get(&table, 9999).unwrap(), None);

        remove_dir_all(db_path).unwrap();
    }

    #[test]
    fn test_update() {
        let (db, table, db_path) = setup_db();

        let pos = Position {
            x: 0.0,
            y: 64.0,
            z: 0.0,
        };
        let data = TestData {
            dimension: "Nether".into(),
            pos: pos.clone(),
        };
        let updated = TestData {
            dimension: "Overworld".into(),
            pos: pos.clone(),
        };
        db.insert(&table, 1001, data.clone()).unwrap();

        db.update(&table, 1001, updated.clone()).unwrap();

        assert_eq!(db.get(&table, 1001).unwrap(), Some(updated));
        remove_dir_all(db_path).unwrap();
    }

    #[test]
    fn test_upsert() {
        let (db, table, db_path) = setup_db();

        let pos = Position {
            x: 0.0,
            y: 64.0,
            z: 0.0,
        };
        let data = TestData {
            dimension: "Nether".into(),
            pos: pos.clone(),
        };
        let updated = TestData {
            dimension: "Overworld".into(),
            pos: pos.clone(),
        };
        db.upsert(&table, 1001, &data.clone()).unwrap();
        assert_eq!(db.get(&table, 1001).unwrap(), Some(data.clone()));

        db.upsert(&table, 1001, &updated.clone()).unwrap();
        assert_eq!(db.get(&table, 1001).unwrap(), Some(updated));
        remove_dir_all(db_path).unwrap();
    }

    #[test]
    fn test_delete() {
        let (db, table, db_path) = setup_db();

        let pos = Position {
            x: 0.0,
            y: 64.0,
            z: 0.0,
        };
        let data = TestData {
            dimension: "Nether".into(),
            pos: pos.clone(),
        };
        db.insert(&table, 1001, data).unwrap();

        db.delete(&table, 1001).unwrap();
        assert_eq!(db.get(&table, 1001).unwrap(), None);
        remove_dir_all(db_path).unwrap();
    }
}
