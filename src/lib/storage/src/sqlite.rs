use rusqlite::{params, params_from_iter, Connection, Row};
use std::{collections::HashMap, marker::PhantomData, path::PathBuf};

use crate::errors::StorageError;

// TODO: Implement proper error mapping
impl From<rusqlite::Error> for StorageError {
    fn from(err: rusqlite::Error) -> Self {
        StorageError::DatabaseError(err.to_string())
    }
}

/// Trait that types must implement to be stored in SqliteDatabase
/// This defines how the type maps to SQL columns
pub trait SqlStorable: Sized {
    /// Returns the SQL schema for creating the table (without CREATE TABLE and table name)
    /// Example: "key TEXT PRIMARY KEY, x REAL, y REAL, z REAL"
    fn schema() -> &'static str;

    /// Returns the column names for INSERT/UPDATE (excluding key)
    /// Example: "x, y, z"
    fn columns() -> &'static str;

    /// Returns the placeholder string for VALUES clause
    /// Example: "?1, ?2, ?3" for 3 columns (key is always ?1)
    fn value_placeholders() -> &'static str;

    /// Binds the values to a statement (starting from index 2, since 1 is the key)
    fn bind_values(
        &self,
        stmt: &mut rusqlite::Statement,
        start_idx: usize,
    ) -> Result<(), StorageError>;

    /// Reads a row from the database and constructs Self
    fn from_row(row: &Row) -> Result<Self, StorageError>;
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

impl<T> SqliteDatabase<T>
where
    T: SqlStorable,
{
    pub fn create_table(&self, table: &str) -> Result<(), StorageError> {
        let conn = self.open_conn()?;
        let sql = format!("CREATE TABLE IF NOT EXISTS \"{}\" ({})", table, T::schema());
        conn.execute(&sql, [])?;
        Ok(())
    }

    pub fn insert(&self, table: &str, key: u128, value: T) -> Result<(), StorageError> {
        let conn = self.open_conn()?;
        let sql = format!(
            "INSERT INTO \"{}\" (key, {}) VALUES (?1, {})",
            table,
            T::columns(),
            T::value_placeholders()
        );
        let mut stmt = conn.prepare(&sql)?;
        stmt.raw_bind_parameter(1, key.to_string())?;
        value.bind_values(&mut stmt, 2)?;
        stmt.raw_execute()?;
        Ok(())
    }

    pub fn get(&self, table: &str, key: u128) -> Result<Option<T>, StorageError> {
        let conn = self.open_conn()?;
        let sql = format!("SELECT * FROM \"{}\" WHERE key = ?1 LIMIT 1", table);
        let mut stmt = conn.prepare(&sql)?;
        let mut rows = stmt.query(params![key.to_string()])?;
        if let Some(row) = rows.next()? {
            let value = T::from_row(row)?;
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }

    pub fn delete(&self, table: &str, key: u128) -> Result<(), StorageError> {
        let conn = self.open_conn()?;
        let sql = format!("DELETE FROM \"{}\" WHERE key = ?1", table);
        conn.execute(&sql, params![key.to_string()])?;
        Ok(())
    }

    pub fn update(&self, table: &str, key: u128, value: T) -> Result<(), StorageError> {
        let conn = self.open_conn()?;

        // Build SET clause dynamically based on columns
        let columns = T::columns();
        let column_list: Vec<&str> = columns.split(',').map(|s| s.trim()).collect();
        let set_clause = column_list
            .iter()
            .enumerate()
            .map(|(i, col)| format!("{} = ?{}", col, i + 2))
            .collect::<Vec<_>>()
            .join(", ");

        let sql = format!("UPDATE \"{}\" SET {} WHERE key = ?1", table, set_clause);
        let mut stmt = conn.prepare(&sql)?;
        stmt.raw_bind_parameter(1, key.to_string())?;
        value.bind_values(&mut stmt, 2)?;
        stmt.raw_execute()?;
        Ok(())
    }

    pub fn upsert(&self, table: &str, key: u128, value: &T) -> Result<bool, StorageError> {
        let conn = self.open_conn()?;

        // Build SET clause for UPDATE part
        let columns = T::columns();
        let column_list: Vec<&str> = columns.split(',').map(|s| s.trim()).collect();
        let set_clause = column_list
            .iter()
            .map(|col| format!("{} = excluded.{}", col, col))
            .collect::<Vec<_>>()
            .join(", ");

        let sql = format!(
            "INSERT INTO \"{}\" (key, {}) VALUES (?1, {})
             ON CONFLICT(key) DO UPDATE SET {}",
            table,
            T::columns(),
            T::value_placeholders(),
            set_clause
        );
        let mut stmt = conn.prepare(&sql)?;
        stmt.raw_bind_parameter(1, key.to_string())?;
        value.bind_values(&mut stmt, 2)?;
        stmt.raw_execute()?;
        Ok(true)
    }

    pub fn batch_insert(&self, table: &str, data: Vec<(u128, T)>) -> Result<(), StorageError> {
        if data.is_empty() {
            return Ok(());
        }
        let mut conn = self.open_conn()?;
        let tx = conn.transaction()?;
        let sql = format!(
            "INSERT INTO \"{}\" (key, {}) VALUES (?1, {})",
            table,
            T::columns(),
            T::value_placeholders()
        );
        {
            for (k, v) in data {
                let mut stmt = tx.prepare(&sql)?;
                stmt.raw_bind_parameter(1, k.to_string())?;
                v.bind_values(&mut stmt, 2)?;
                stmt.raw_execute()?;
            }
        }
        tx.commit()?;
        Ok(())
    }

    pub fn batch_get(&self, table: &str, keys: Vec<u128>) -> Result<Vec<Option<T>>, StorageError> {
        if keys.is_empty() {
            return Ok(Vec::new());
        }
        let conn = self.open_conn()?;
        let placeholders = std::iter::repeat_n("?", keys.len())
            .collect::<Vec<_>>()
            .join(",");
        let query_sql = format!(
            "SELECT * FROM \"{}\" WHERE key IN ({})",
            table, placeholders
        );
        let mut stmt = conn.prepare(&query_sql)?;
        let key_strings = keys.iter().map(|k| k.to_string());
        let mut rows = stmt.query(params_from_iter(key_strings))?;

        let mut found: HashMap<String, T> = HashMap::new();
        while let Some(row) = rows.next()? {
            let key_str: String = row.get(0)?;
            let value = T::from_row(row)?;
            found.insert(key_str, value);
        }

        let mut result = Vec::with_capacity(keys.len());
        for k in keys {
            let ks = k.to_string();
            if let Some(value) = found.remove(&ks) {
                result.push(Some(value));
            } else {
                result.push(None);
            }
        }

        Ok(result)
    }

    pub fn batch_upsert(&self, table: &str, data: Vec<(u128, T)>) -> Result<(), StorageError> {
        if data.is_empty() {
            return Ok(());
        }
        let mut conn = self.open_conn()?;
        let tx = conn.transaction()?;

        // Build SET clause for UPDATE part
        let columns = T::columns();
        let column_list: Vec<&str> = columns.split(',').map(|s| s.trim()).collect();
        let set_clause = column_list
            .iter()
            .map(|col| format!("{} = excluded.{}", col, col))
            .collect::<Vec<_>>()
            .join(", ");

        let sql = format!(
            "INSERT INTO \"{}\" (key, {}) VALUES (?1, {})
             ON CONFLICT(key) DO UPDATE SET {}",
            table,
            T::columns(),
            T::value_placeholders(),
            set_clause
        );
        {
            for (k, v) in data {
                let mut stmt = tx.prepare(&sql)?;
                stmt.raw_bind_parameter(1, k.to_string())?;
                v.bind_values(&mut stmt, 2)?;
                stmt.raw_execute()?;
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
    use rusqlite::Row;
    use tempfile::tempdir;

    #[derive(Debug, PartialEq, Clone)]
    struct TestData {
        pub pos: Position,
        pub dimension: String,
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct Position {
        pub x: f64,
        pub y: f64,
        pub z: f64,
    }

    impl SqlStorable for TestData {
        fn schema() -> &'static str {
            "key TEXT PRIMARY KEY, pos_x REAL NOT NULL, pos_y REAL NOT NULL, pos_z REAL NOT NULL, dimension TEXT NOT NULL"
        }

        fn columns() -> &'static str {
            "pos_x, pos_y, pos_z, dimension"
        }

        fn value_placeholders() -> &'static str {
            "?2, ?3, ?4, ?5"
        }

        fn bind_values(
            &self,
            stmt: &mut rusqlite::Statement,
            start_idx: usize,
        ) -> Result<(), StorageError> {
            stmt.raw_bind_parameter(start_idx, self.pos.x)?;
            stmt.raw_bind_parameter(start_idx + 1, self.pos.y)?;
            stmt.raw_bind_parameter(start_idx + 2, self.pos.z)?;
            stmt.raw_bind_parameter(start_idx + 3, self.dimension.as_str())?;
            Ok(())
        }

        fn from_row(row: &Row) -> Result<Self, StorageError> {
            Ok(TestData {
                pos: Position {
                    x: row.get(1)?,
                    y: row.get(2)?,
                    z: row.get(3)?,
                },
                dimension: row.get(4)?,
            })
        }
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
