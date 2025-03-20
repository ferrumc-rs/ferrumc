use criterion::{black_box, Criterion};
use ferrumc_core::collisions::bounds::CollisionBounds;

pub fn bench_collides(c: &mut Criterion) {
    let mut g = c.benchmark_group("collisions");
    g.bench_function("Simple collides", |b| {
        b.iter(|| {
            let bounds1 = black_box(CollisionBounds {
                x_offset_start: 0.0,
                x_offset_end: 2.0,
                y_offset_start: 0.0,
                y_offset_end: 2.0,
                z_offset_start: 0.0,
                z_offset_end: 2.0,
            });
            let bounds2 = black_box(CollisionBounds {
                x_offset_start: 1.0,
                x_offset_end: 3.0,
                y_offset_start: 1.0,
                y_offset_end: 3.0,
                z_offset_start: 1.0,
                z_offset_end: 3.0,
            });
            bounds1.collides(
                black_box((0.0, 0.0, 0.0)),
                &bounds2,
                black_box((0.0, 0.0, 0.0)),
            );
        })
    });
    g.bench_function("Complex collides", |b| {
        b.iter(|| {
            let bounds1 = black_box(CollisionBounds {
                x_offset_start: 64.2,
                x_offset_end: -8.4,
                y_offset_start: -12.0,
                y_offset_end: 16.3,
                z_offset_start: 99.55,
                z_offset_end: 100.999,
            });
            let bounds2 = black_box(CollisionBounds {
                x_offset_start: 5.0,
                x_offset_end: 6.0,
                y_offset_start: 1.0,
                y_offset_end: 0.0,
                z_offset_start: 2.0,
                z_offset_end: 3.0,
            });
            bounds1.collides(
                black_box((12.0, 66.0, -5.0)),
                &bounds2,
                black_box((4444.0, -300.0, 0.1)),
            );
        })
    });
}
