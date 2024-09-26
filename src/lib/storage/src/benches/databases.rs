use criterion::black_box;
use criterion::Criterion;
use ferrumc_storage::backends::redb::RedbBackend;
use ferrumc_storage::errors::StorageError;
use ferrumc_storage::DatabaseBackend;
use ferrumc_utils::root;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::AtomicU64;
use lazy_static::lazy_static;
use parking_lot::Mutex;
use tokio::fs::File;
use tokio::runtime::Runtime;
use ferrumc_storage::backends::surrealkv::SurrealKVBackend;

lazy_static!(
    static ref LASTKEY: AtomicU64 = AtomicU64::new(0);
);

fn gen_key() -> u64 {
    LASTKEY.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
}

pub fn database_benchmarks(c: &mut Criterion) {
    let data = std::fs::read(root!(".etc/codec.nbt")).unwrap();
    let runtime = Runtime::new().unwrap();
    let _handle = runtime.enter();
    {
        let mut temps = Vec::new();

        let mut redb_backend = runtime.block_on(async {
            let mut db_file = PathBuf::from(root!(".temp/redb"));
            if !db_file.exists() {
                std::fs::create_dir_all(&db_file).unwrap();
            }
            db_file.push("test.db");
            let mut backend = RedbBackend::initialize(Some(db_file.clone())).await.unwrap();
            backend.create_table("test".to_string()).await.unwrap();
            temps.push(db_file);
            backend
        });

        let mut surreal_backend = runtime.block_on(async {
            let mut db_file = PathBuf::from(root!(".temp/surreal"));
            if !db_file.exists() {
                std::fs::create_dir_all(&db_file).unwrap();
            }
            db_file.push("test.db");
            let backend = SurrealKVBackend::initialize(Some(db_file.clone())).await.unwrap();
            temps.push(db_file);
            backend
        });

        let mut write_group = c.benchmark_group("Write");
        write_group.measurement_time(std::time::Duration::from_secs(10));
        write_group.throughput(criterion::Throughput::Bytes(data.len() as u64));
        write_group.bench_with_input("Redb", &("test".to_string(), data.clone()), |b, (table, data)| {
            b.iter(|| {
                runtime.block_on(redb_backend.upsert(table.clone(), gen_key(), data.clone())).unwrap();
            });
        });
        write_group.bench_with_input("SurrealKV", &("test".to_string(), data.clone()), |b, (table, data)| {
            b.iter(|| {
                runtime.block_on(surreal_backend.insert(table.clone(), gen_key(), data.clone())).unwrap();
            });
        });
        write_group.finish();

        let mut read_group = c.benchmark_group("Read");

        read_group.throughput(criterion::Throughput::Bytes(data.len() as u64));
        read_group.measurement_time(std::time::Duration::from_secs(10));
        runtime.block_on(redb_backend.insert("test".to_string(), 0, data.clone())).unwrap();
        runtime.block_on(surreal_backend.insert("test".to_string(), 0, data.clone())).unwrap();

        read_group.bench_with_input("Redb", &("test".to_string(), 0), |b, (table, key)| {
            b.iter(|| {
                black_box(runtime.block_on(redb_backend.get(table.clone(), *key)).unwrap());
            });
        });
        read_group.bench_with_input("SurrealKV", &("test".to_string(), 0), |b, (table, key)| {
            b.iter(|| {
                black_box(runtime.block_on(surreal_backend.get(table.clone(), *key)).unwrap());
            });
        });

        read_group.finish();


        runtime.block_on(surreal_backend.close()).unwrap();


        temps.iter().for_each(|temp_dir| {
            match temp_dir.is_dir() {
                true => {
                    std::fs::remove_dir_all(temp_dir).unwrap();
                }
                false => {
                    std::fs::remove_file(temp_dir).unwrap();
                }
            }
        });
    }
}