use ferrumc_storage::lmdb::LmdbBackend;
use rand::Rng;
use std::collections::HashSet;

fn generate_random_data(size: usize) -> Vec<u8> {
    (0..size).map(|_| rand::random::<u8>()).collect()
}

fn generate_random_key(used: &mut HashSet<u128>) -> u128 {
    let mut key = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    while used.contains(&key) {
        key += 1;
    }
    used.insert(key);
    key
}

fn select_random<T: Clone + Copy>(choices: Vec<T>) -> T {
    let mut rng = rand::rng();
    let index = rng.random::<u32>() as usize % choices.len();
    *choices.get(index).unwrap()
}

pub(crate) fn db_benches(c: &mut criterion::Criterion) {
    let mut used_keys = HashSet::new();
    let tempdir = tempfile::TempDir::new().unwrap().keep();

    let db = LmdbBackend::initialize(Some(tempdir), 100 * 1024 * 1024 * 1024).unwrap();

    db.create_table("insert_test".to_string()).unwrap();

    let mut insert_group = c.benchmark_group("Insert");

    insert_group.bench_function("512b".to_string(), |b| {
        b.iter(|| {
            db.insert(
                "insert_test".to_string(),
                generate_random_key(&mut used_keys),
                generate_random_data(512),
            )
            .unwrap();
        })
    });

    insert_group.bench_function("1kb".to_string(), |b| {
        b.iter(|| {
            db.insert(
                "insert_test".to_string(),
                generate_random_key(&mut used_keys),
                generate_random_data(1024),
            )
            .unwrap();
        })
    });

    insert_group.bench_function("4kb".to_string(), |b| {
        b.iter(|| {
            db.insert(
                "insert_test".to_string(),
                generate_random_key(&mut used_keys),
                generate_random_data(4096),
            )
            .unwrap();
        })
    });

    insert_group.finish();

    let mut read_group = c.benchmark_group("Read");

    db.create_table("read_test".to_string()).unwrap();

    let keys_512b = (0..1000)
        .map(|_| generate_random_key(&mut used_keys))
        .collect::<Vec<_>>();

    for key in keys_512b.iter() {
        db.insert("read_test".to_string(), *key, generate_random_data(512))
            .unwrap();
    }

    read_group.bench_function("512b".to_string(), |b| {
        b.iter(|| {
            db.get("read_test".to_string(), select_random(keys_512b.clone()))
                .unwrap();
        })
    });

    let keys_1kb = (0..1000)
        .map(|_| generate_random_key(&mut used_keys))
        .collect::<Vec<_>>();

    for key in keys_1kb.iter() {
        db.insert("read_test".to_string(), *key, generate_random_data(1024))
            .unwrap();
    }

    read_group.bench_function("1kb".to_string(), |b| {
        b.iter(|| {
            db.get("read_test".to_string(), select_random(keys_1kb.clone()))
                .unwrap();
        })
    });

    let keys_4kb = (0..1000)
        .map(|_| generate_random_key(&mut used_keys))
        .collect::<Vec<_>>();

    for key in keys_4kb.iter() {
        db.insert("read_test".to_string(), *key, generate_random_data(4096))
            .unwrap();
    }

    read_group.bench_function("4kb".to_string(), |b| {
        b.iter(|| {
            db.get("read_test".to_string(), select_random(keys_4kb.clone()))
                .unwrap();
        })
    });

    read_group.finish();
}
