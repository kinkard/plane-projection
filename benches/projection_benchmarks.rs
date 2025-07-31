use criterion::{Criterion, criterion_group, criterion_main};
use plane_projection::PlaneProjection;
use std::hint::black_box;

fn bench_distance(c: &mut Criterion) {
    c.bench_function("distance", |b| {
        b.iter(|| {
            let projection = PlaneProjection::new(black_box(55.65));
            black_box(projection.distance(black_box((55.60, 13.5)), black_box((55.61, 13.53))))
        });
    });

    c.bench_function("reused projection distance", |b| {
        let projection = PlaneProjection::new(55.65);
        b.iter(|| {
            black_box(projection.distance(black_box((55.60, 13.5)), black_box((55.61, 13.53))))
        });
    });
}

fn bench_distance_to_segment(c: &mut Criterion) {
    // Points in circle to check all branches for the "distance to segment" function
    let base = (55.60, 13.5);
    let segment = ((base.0, base.0 - 0.5), (base.0, base.0 + 0.5));
    let points = (0..100)
        .map(|i| {
            let angle = 2.0 * std::f64::consts::PI * i as f64 / 100.0;
            (base.0 + angle.cos(), base.1 + angle.sin())
        })
        .collect::<Vec<(f64, f64)>>();

    c.bench_function("distance to segment", |b| {
        b.iter(|| {
            for point in &points {
                let projection = PlaneProjection::new(black_box(base.0));
                black_box(projection.distance_to_segment(black_box(*point), black_box(segment)));
            }
            // add degenerate cases
            let projection = PlaneProjection::new(black_box(base.0));
            black_box(
                projection.distance_to_segment(black_box(base), black_box((segment.0, segment.0))),
            );
            let projection = PlaneProjection::new(black_box(base.0));
            black_box(
                projection.distance_to_segment(black_box(base), black_box((segment.1, segment.1))),
            );
        });
    });

    c.bench_function("reused projection distance to segment", |b| {
        let projection = PlaneProjection::new(black_box(base.0));
        b.iter(|| {
            for point in &points {
                black_box(projection.distance_to_segment(black_box(*point), black_box(segment)));
            }
            black_box(
                projection.distance_to_segment(black_box(base), black_box((segment.0, segment.0))),
            );
            black_box(
                projection.distance_to_segment(black_box(base), black_box((segment.1, segment.1))),
            );
        });
    });
}

fn bench_heading(c: &mut Criterion) {
    c.bench_function("heading", |b| {
        b.iter(|| {
            let projection = PlaneProjection::new(black_box(55.65));
            black_box(projection.heading(
                black_box((55.60330902847681, 13.001973666557435)),
                black_box((55.704141722528554, 13.191304107330561)),
            ))
        });
    });

    c.bench_function("reused projection heading", |b| {
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
    bench_distance,
    bench_distance_to_segment,
    bench_heading,
);
criterion_main!(benches);
