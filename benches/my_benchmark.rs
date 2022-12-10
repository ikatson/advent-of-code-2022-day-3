use std::io::Read;

use ad3p2::{s1, s2, s3};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let mut f = std::fs::File::open("input.txt").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
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
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
