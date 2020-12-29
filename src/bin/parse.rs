use loki_bench;
use std::env;
use std::fs;

fn main() {
    let data = fs::read_to_string("data/small_access.log").unwrap();

    match env::args().nth(1).as_ref() {
        Some(s) if s == "nom" => {
            let p = loki_bench::nom::CommonLogParser { input: &data };
            assert_eq!(p.count(), 161761);
        },
        Some(s) if s == "regex" => {
            let p = loki_bench::regex::CommonLogParser::new(&data);
            assert_eq!(p.count(), 161761);
        },
        _ => panic!("Please specify which parser to use: nom, regex.")
    }
}