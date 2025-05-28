use criterion::{Criterion, criterion_group, criterion_main};
use plane_projection::PlaneProjection;
use std::hint::black_box;

fn bench_plane_projection_distance(c: &mut Criterion) {
    c.bench_function("plane_projection_distance", |b| {
        b.iter(|| {
            let projection = PlaneProjection::new(black_box(55.65));
            black_box(projection.distance(black_box((55.60, 13.5)), black_box((55.61, 13.53))))
        });
    });

    c.bench_function("reused_plane_projection_distance", |b| {
        let projection = PlaneProjection::new(55.65);
        b.iter(|| {
            black_box(projection.distance(black_box((55.60, 13.5)), black_box((55.61, 13.53))))
        });
    });
}

fn bench_plane_projection_heading(c: &mut Criterion) {
    c.bench_function("plane_projection_heading", |b| {
        b.iter(|| {
            let projection = PlaneProjection::new(black_box(55.65));
            black_box(projection.heading(
                black_box((55.60330902847681, 13.001973666557435)),
                black_box((55.704141722528554, 13.191304107330561)),
            ))
        });
    });

    c.bench_function("reused_plane_projection_heading", |b| {
        let projection = PlaneProjection::new(55.65);
        b.iter(|| {
            black_box(projection.heading(
                black_box((55.60330902847681, 13.001973666557435)),
                black_box((55.704141722528554, 13.191304107330561)),
            ))
        });
    });
}

criterion_group!(
    benches,
    bench_plane_projection_distance,
    bench_plane_projection_heading,
);
criterion_main!(benches);
