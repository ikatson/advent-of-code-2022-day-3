use std::io::Read;

use ad3p2::{s1, s2, s3};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let mut f = std::fs::File::open("input.txt").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();

    let lines: Vec<Vec<u8>> = ad3p2::lsplit::l2(s.as_bytes())
        .map(|l| l.to_owned())
        .collect();

    c.bench_function("process_buf::s1", |b| {
        b.iter(|| s1::process_buf(black_box(s.as_bytes())))
    });
    c.bench_function("process_buf::s2", |b| {
        b.iter(|| s2::process_buf(black_box(s.as_bytes())))
    });
    c.bench_function("process_buf::s3", |b| {
        b.iter(|| s3::process_buf(black_box(s.as_bytes())))
    });
    c.bench_function("process_buf::s1_part2", |b| {
        b.iter(|| s1::process_buf_part_2(black_box(s.as_bytes())))
    });
    c.bench_function("process_buf::s2_part2", |b| {
        b.iter(|| s2::process_buf_part_2(black_box(s.as_bytes())))
    });
    c.bench_function("process_buf::s3_part2", |b| {
        b.iter(|| s3::process_buf_part_2(black_box(s.as_bytes())))
    });

    c.bench_function("process_buf::s3_part2 aligned with c3", |b| {
        b.iter(|| {
            ad3p2::part2::process_buf_generic(
                lines.iter().map(|l| l.as_ref()),
                ad3p2::compartment::c3,
            )
        });
    });

    c.bench_function("process_buf::s3_part2 aligned with simd c2", |b| {
        b.iter(|| {
            ad3p2::part2::process_buf_generic(
                lines.iter().map(|l| l.as_ref()),
                ad3p2::compartment::c2,
            )
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
