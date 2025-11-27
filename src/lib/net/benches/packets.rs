use criterion::measurement::WallTime;
use ferrumc_protocol::codec::encode::{NetEncode, NetEncodeOpts};
use std::hint::black_box;

pub fn bench_packets(c: &mut criterion::BenchmarkGroup<WallTime>) {
    bench_chunk_packet(c);
}

fn bench_chunk_packet(c: &mut criterion::BenchmarkGroup<WallTime>) {
    let chunk = ferrumc_world_gen::WorldGenerator::new(0)
        .generate_chunk(0, 0)
        .unwrap();
    let chunk_packet = black_box(
        ferrumc_net::packets::outgoing::chunk_and_light_data::ChunkAndLightData::from_chunk(&chunk)
            .unwrap(),
    );
    c.bench_function("chunk_and_light_data", |b| {
        b.iter(|| {
            let mut buffer = Vec::new();
            chunk_packet
                .encode(&mut buffer, &NetEncodeOpts::WithLength)
                .unwrap();
        });
    });
}
