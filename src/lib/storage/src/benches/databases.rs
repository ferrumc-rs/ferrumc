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

lazy_static!(
    static ref LASTKEY: AtomicU64 = AtomicU64::new(0);
);

fn gen_key() -> u64 {
    LASTKEY.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
}

pub fn database_benchmarks(c: &mut Criterion) {

    let data = std::fs::read(root!(".etc/codec.nbt")).unwrap();
    let runtime = Runtime::new().unwrap();
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

    let mut write_group = c.benchmark_group("Write");
    let runtime = Runtime::new().unwrap();
    write_group.bench_with_input("Redb", &("test".to_string(), data.clone()), |b, (table, data)| {
        b.iter(|| {
            runtime.block_on(redb_backend.upsert(table.clone(), gen_key(), data.clone())).unwrap();
        });
    });
    write_group.finish();

    let mut read_group = c.benchmark_group("Read");
    let runtime = Runtime::new().unwrap();
    
    runtime.block_on(redb_backend.insert("test".to_string(), 0, data.clone())).unwrap();
    
    read_group.bench_with_input("Redb", &("test".to_string(), 0), |b, (table, key)| {
        b.iter(|| {
            black_box(runtime.block_on(redb_backend.get(table.clone(), *key)).unwrap());
        });
    });
    
    read_group.finish();

    temps.iter().for_each(|temp_dir| {
        std::fs::remove_file(temp_dir).unwrap();
    });
}