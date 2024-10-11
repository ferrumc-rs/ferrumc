use criterion::black_box;
use ferrumc_storage::{
    compressors::{
        brotli::BrotliCompressor, deflate::DeflateCompressor, gzip::GzipCompressor,
        zlib::ZlibCompressor, zstd::ZstdCompressor,
    },
    Compressor,
};
use ferrumc_utils::root;

fn zstd_compress(data: &[u8]) {
    let compressor = ZstdCompressor::new(6);
    let compressed = compressor.compress(data).unwrap();
    black_box(compressed);
}

fn zstd_decompress(data: &[u8]) {
    let compressor = ZstdCompressor::new(0);
    let decompressed = compressor.decompress(data).unwrap();
    black_box(decompressed);
}

fn gzip_compress(data: &[u8]) {
    let compressor = GzipCompressor::new(6);
    let compressed = compressor.compress(data).unwrap();
    black_box(compressed);
}

fn gzip_decompress(data: &[u8]) {
    let compressor = GzipCompressor::new(0);
    let decompressed = compressor.decompress(data).unwrap();
    black_box(decompressed);
}

fn deflate_compress(data: &[u8]) {
    let compressor = DeflateCompressor::new(6);
    let compressed = compressor.compress(data).unwrap();
    black_box(compressed);
}

fn deflate_decompress(data: &[u8]) {
    let compressor = DeflateCompressor::new(6);
    let decompressed = compressor.decompress(data).unwrap();
    black_box(decompressed);
}

fn zlib_compress(data: &[u8]) {
    let compressor = ZlibCompressor::new(6);
    let compressed = compressor.compress(data).unwrap();
    black_box(compressed);
}

fn zlib_decompress(data: &[u8]) {
    let compressor = ZlibCompressor::new(0);
    let decompressed = compressor.decompress(data).unwrap();
    black_box(decompressed);
}

fn brotli_compress(data: &[u8]) {
    let compressor = BrotliCompressor::new(6);
    let compressed = compressor.compress(data).unwrap();
    black_box(compressed);
}

fn brotli_decompress(data: &[u8]) {
    let compressor = BrotliCompressor::new(0);
    let decompressed = compressor.decompress(data).unwrap();
    black_box(decompressed);
}

pub fn compression_benchmarks(c: &mut criterion::Criterion) {
    let data = std::fs::read(root!(".etc/codec.nbt")).unwrap();
    let data = data.as_slice();

    let mut compress_group = c.benchmark_group("Compression");
    compress_group.throughput(criterion::Throughput::Bytes(data.len() as u64));
    compress_group.bench_function("Zstd", |b| b.iter(|| zstd_compress(black_box(data))));
    compress_group.bench_function("Gzip", |b| b.iter(|| gzip_compress(black_box(data))));
    compress_group.bench_function("Deflate", |b| b.iter(|| deflate_compress(black_box(data))));
    compress_group.bench_function("Zlib", |b| b.iter(|| zlib_compress(black_box(data))));
    compress_group.bench_function("Brotli", |b| b.iter(|| brotli_compress(black_box(data))));
    compress_group.finish();

    let zstd_compressor = ZstdCompressor::new(6);
    let zstd_compressed = zstd_compressor.compress(data).unwrap();
    let gzip_compressor = GzipCompressor::new(6);
    let gzip_compressed = gzip_compressor.compress(data).unwrap();
    let deflate_compressor = DeflateCompressor::new(6);
    let deflate_compressed = deflate_compressor.compress(data).unwrap();
    let zlib_compressor = ZlibCompressor::new(6);
    let zlib_compressed = zlib_compressor.compress(data).unwrap();
    let brotli_compressor = BrotliCompressor::new(6);
    let brotli_compressed = brotli_compressor.compress(data).unwrap();

    let mut decompress_group = c.benchmark_group("Decompression");
    decompress_group.throughput(criterion::Throughput::Bytes(data.len() as u64));
    decompress_group.bench_function("Zstd", |b| {
        b.iter(|| zstd_decompress(black_box(zstd_compressed.as_slice())))
    });
    decompress_group.bench_function("Gzip", |b| {
        b.iter(|| gzip_decompress(black_box(gzip_compressed.as_slice())))
    });
    decompress_group.bench_function("Deflate", |b| {
        b.iter(|| deflate_decompress(black_box(deflate_compressed.as_slice())))
    });
    decompress_group.bench_function("Zlib", |b| {
        b.iter(|| zlib_decompress(black_box(zlib_compressed.as_slice())))
    });
    decompress_group.bench_function("Brotli", |b| {
        b.iter(|| brotli_decompress(black_box(brotli_compressed.as_slice())))
    });
    decompress_group.finish();

    let mut roundtrip_group = c.benchmark_group("Roundtrip");
    roundtrip_group.throughput(criterion::Throughput::Bytes(data.len() as u64));
    roundtrip_group.bench_function("Zstd", |b| {
        b.iter(|| {
            let compressed = zstd_compressor.compress(data).unwrap();
            let decompressed = zstd_compressor.decompress(compressed.as_slice()).unwrap();
            black_box(decompressed);
        })
    });
    roundtrip_group.bench_function("Gzip", |b| {
        b.iter(|| {
            let compressed = gzip_compressor.compress(data).unwrap();
            let decompressed = gzip_compressor.decompress(compressed.as_slice()).unwrap();
            black_box(decompressed);
        })
    });
    roundtrip_group.bench_function("Deflate", |b| {
        b.iter(|| {
            let compressed = deflate_compressor.compress(data).unwrap();
            let decompressed = deflate_compressor
                .decompress(compressed.as_slice())
                .unwrap();
            black_box(decompressed);
        })
    });
    roundtrip_group.bench_function("Zlib", |b| {
        b.iter(|| {
            let compressed = zlib_compressor.compress(data).unwrap();
            let decompressed = zlib_compressor.decompress(compressed.as_slice()).unwrap();
            black_box(decompressed);
        })
    });
    roundtrip_group.bench_function("Brotli", |b| {
        b.iter(|| {
            let compressed = brotli_compressor.compress(data).unwrap();
            let decompressed = brotli_compressor.decompress(compressed.as_slice()).unwrap();
            black_box(decompressed);
        })
    });
    roundtrip_group.finish();
}
