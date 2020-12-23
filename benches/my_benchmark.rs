use loki_bench;

use criterion::{criterion_group, criterion_main, Criterion};
use std::fs;

fn nom_parser(c: &mut Criterion) {
    let data = fs::read_to_string("data/small_access.log").unwrap();
    c.bench_function("nom", move |b| {
        b.iter(|| {
            let p = loki_bench::nom::CommonLogParser { input: &data };
            assert_eq!(p.count(), 161761);
        });
    });
}

fn regex_parser(c: &mut Criterion) {
    let data = fs::read_to_string("data/small_access.log").unwrap();
    c.bench_function("regex", move |b| {
        b.iter(|| {
            let p = loki_bench::regex::CommonLogParser::new(&data);
            assert_eq!(p.count(), 161761);
        });
    });
}

criterion_group!(benches, nom_parser, regex_parser);
criterion_main!(benches);
