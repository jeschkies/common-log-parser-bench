use std::fs;

fn main() {
    let data = fs::read_to_string("data/small_access.log").unwrap();
    //let p = loki_bench::nom::CommonLogParser { input: &data };
    let p = loki_bench::regex::CommonLogParser::new(&data);
    assert_eq!(p.count(), 161761);
}