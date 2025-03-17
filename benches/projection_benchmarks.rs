use criterion::{Criterion, black_box, criterion_group, criterion_main};
use plane_projection::PlaneProjection;

fn bench_plane_projection_distance(c: &mut Criterion) {
    c.bench_function("reused_plane_projection", |b| {
        let projection = PlaneProjection::new(55.65);
        b.iter(|| {
            black_box(projection.distance(black_box((55.60, 13.5)), black_box((55.61, 13.53))))
        });
    });

    c.bench_function("single_shot_plane_projection", |b| {
        b.iter(|| {
            let projection = PlaneProjection::new(black_box(55.65));
            black_box(projection.distance(black_box((55.60, 13.5)), black_box((55.61, 13.53))))
        });
    });
}

criterion_group!(benches, bench_plane_projection_distance,);
criterion_main!(benches);
