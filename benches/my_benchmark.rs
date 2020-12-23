use loki_bench;

use criterion::{criterion_group, criterion_main, Criterion};
use std::fs;

fn nom_parser(c: &mut Criterion) {
    let data = fs::read_to_string("data/small_access.log").unwrap();
    let p = loki_bench::nom::CommonLogParser { input: &data };
    c.bench_function("nom", move |b| {
        // TODO: parse more than first line.
        b.iter(|| assert_eq!(p.count(), 161761));
    });
}

criterion_group!(benches, nom_parser);
criterion_main!(benches);
