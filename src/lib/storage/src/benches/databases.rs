use criterion::black_box;
use criterion::Criterion;
use ferrumc_storage::backends::redb::RedbBackend;
use ferrumc_storage::backends::surrealkv::SurrealKVBackend;
use ferrumc_storage::DatabaseBackend;
use ferrumc_utils::root;
use lazy_static::lazy_static;
use rand::prelude::IndexedRandom;
use std::path::PathBuf;
use std::sync::atomic::AtomicU64;
use tokio::runtime::Runtime;

const READ_KEYS: u32 = 2048;

lazy_static! {
    static ref KEY: AtomicU64 = AtomicU64::new(0);
}

fn gen_key() -> u64 {
    KEY.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
}

fn random_key() -> u64 {
    rand::random::<u64>()
}

pub fn database_benchmarks(c: &mut Criterion) {
    let data = std::fs::read(root!(".etc/codec.zip")).unwrap();
    let runtime = Runtime::new().unwrap();
    let _handle = runtime.enter();
    {
        let mut temps = Vec::new();

        let mut redb_backend = runtime.block_on(async {
            let mut db_file = PathBuf::from("R:/temp/redb");
            if !db_file.exists() {
                std::fs::create_dir_all(&db_file).unwrap();
            }
            db_file.push("test.db");
            let mut backend = RedbBackend::initialize(Some(db_file.clone()))
                .await
                .unwrap();
            backend.create_table("test".to_string()).await.unwrap();
            temps.push(db_file);
            backend
        });

        let mut surreal_backend = runtime.block_on(async {
            let mut db_file = PathBuf::from("R:/temp/surreal");
            if !db_file.exists() {
                std::fs::create_dir_all(&db_file).unwrap();
            }
            db_file.push("test.db");
            let backend = SurrealKVBackend::initialize(Some(db_file.clone()))
                .await
                .unwrap();
            temps.push(db_file);
            backend
        });

        let mut sled_backend = runtime.block_on(async {
            let mut db_file = PathBuf::from("R:/temp/sled");
            if !db_file.exists() {
                std::fs::create_dir_all(&db_file).unwrap();
            }
            db_file.push("test.db");
            let backend =
                ferrumc_storage::backends::sled::SledBackend::initialize(Some(db_file.clone()))
                    .await
                    .unwrap();
            temps.push(db_file);
            backend
        });

        let mut rocksdb_backend = runtime.block_on(async {
            let mut db_file = PathBuf::from("R:/temp/rocksdb");
            if !db_file.exists() {
                std::fs::create_dir_all(&db_file).unwrap();
            }
            db_file.push("test.db");
            let mut backend = ferrumc_storage::backends::rocksdb::RocksDBBackend::initialize(Some(
                db_file.clone(),
            ))
            .await
            .unwrap();
            temps.push(db_file);
            backend.create_table("test".to_string()).await.unwrap();
            backend
        });

        let mut write_group = c.benchmark_group("Write");
        write_group.measurement_time(std::time::Duration::from_secs(5));
        write_group.throughput(criterion::Throughput::Bytes(data.len() as u64));
        write_group.bench_with_input(
            "Redb",
            &("test".to_string(), data.clone()),
            |b, (table, data)| {
                b.iter(|| {
                    runtime
                        .block_on(redb_backend.upsert(table.clone(), gen_key(), data.clone()))
                        .unwrap();
                });
            },
        );
        write_group.bench_with_input(
            "SurrealKV",
            &("test".to_string(), data.clone()),
            |b, (table, data)| {
                b.iter(|| {
                    runtime
                        .block_on(surreal_backend.insert(table.clone(), gen_key(), data.clone()))
                        .unwrap();
                });
            },
        );
        write_group.bench_with_input(
            "Sled",
            &("test".to_string(), data.clone()),
            |b, (table, data)| {
                b.iter(|| {
                    runtime
                        .block_on(sled_backend.insert(table.clone(), gen_key(), data.clone()))
                        .unwrap();
                });
            },
        );
        write_group.bench_with_input(
            "RocksDB",
            &("test".to_string(), data.clone()),
            |b, (table, data)| {
                b.iter(|| {
                    runtime
                        .block_on(rocksdb_backend.upsert(table.clone(), gen_key(), data.clone()))
                        .unwrap();
                });
            },
        );
        write_group.finish();

        let mut keys = Vec::new();
        for _ in 0..READ_KEYS {
            let key = random_key();
            runtime
                .block_on(redb_backend.insert("test".to_string(), key, data.clone()))
                .unwrap();
            runtime
                .block_on(surreal_backend.insert("test".to_string(), key, data.clone()))
                .unwrap();
            runtime
                .block_on(sled_backend.insert("test".to_string(), key, data.clone()))
                .unwrap();
            runtime
                .block_on(rocksdb_backend.insert("test".to_string(), key, data.clone()))
                .unwrap();
            keys.push(key);
        }

        let mut read_group = c.benchmark_group("Read");

        read_group.throughput(criterion::Throughput::Bytes(data.len() as u64));
        read_group.measurement_time(std::time::Duration::from_secs(10));

        read_group.bench_with_input("Redb", &("test".to_string()), |b, (table)| {
            b.iter(|| {
                let key = keys.choose(&mut rand::thread_rng()).unwrap();
                black_box(
                    runtime
                        .block_on(redb_backend.get(table.clone(), *key))
                        .unwrap(),
                );
            });
        });
        read_group.bench_with_input("SurrealKV", &("test".to_string()), |b, (table)| {
            b.iter(|| {
                let key = keys.choose(&mut rand::thread_rng()).unwrap();
                black_box(
                    runtime
                        .block_on(surreal_backend.get(table.clone(), *key))
                        .unwrap(),
                );
            });
        });
        read_group.bench_with_input("Sled", &("test".to_string()), |b, (table)| {
            b.iter(|| {
                let key = keys.choose(&mut rand::thread_rng()).unwrap();
                black_box(
                    runtime
                        .block_on(sled_backend.get(table.clone(), *key))
                        .unwrap(),
                );
            });
        });
        read_group.bench_with_input("RocksDB", &("test".to_string()), |b, (table)| {
            b.iter(|| {
                let key = keys.choose(&mut rand::thread_rng()).unwrap();
                black_box(
                    runtime
                        .block_on(rocksdb_backend.get(table.clone(), *key))
                        .unwrap(),
                );
            });
        });

        read_group.finish();

        runtime.block_on(surreal_backend.close()).unwrap();

        temps.iter().for_each(|temp_dir| {
            let res = match temp_dir.is_dir() {
                true => std::fs::remove_dir_all(temp_dir),
                false => std::fs::remove_file(temp_dir),
            };
            if let Err(e) = res {
                eprintln!("Failed to remove temp dir: {}", e);
            }
        });
    }
}
