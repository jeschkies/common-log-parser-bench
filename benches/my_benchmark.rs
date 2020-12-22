use loki_bench;

use criterion::{criterion_group, criterion_main, Criterion};
use std::fs;

fn nom_parser(c: &mut Criterion) {
    let data = fs::read_to_string("data/small_access.log").unwrap();
    c.bench_function("nom", move |b| {
        // TODO: parse more than first line.
        b.iter(|| loki_bench::nom::parse(&data).unwrap());
    });
}

criterion_group!(benches, nom_parser);
criterion_main!(benches);
