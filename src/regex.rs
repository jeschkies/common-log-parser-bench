use super::Labels;

use regex::{CaptureMatches, Regex};

lazy_static! {
    static ref RE: Regex = Regex::new(r#"([0-9a-f\.:]+) ([\S\-]+) ([\S\-]+) \[([^\]]+)\] "(.*)" ([0-9]{3}) ([0-9]*).*\n"#).unwrap();
}

pub struct CommonLogParser<'t> {
    captures: CaptureMatches<'static, 't>,
}

impl<'t> CommonLogParser<'t> {
    pub fn new(i: &'t str) -> Self {
        CommonLogParser{
            captures: RE.captures_iter(i),
        }
    }
}

impl<'t> Iterator for CommonLogParser<'t> {
    type Item = Labels<'t>;

    fn next(&mut self) -> Option<Self::Item> {

        if let Some(cap) = self.captures.next() {
            Some(Labels {
                ip: cap.get(1).unwrap().as_str(),
                user: cap.get(2).unwrap().as_str(),
                frank: cap.get(3).unwrap().as_str(),
                date_time: cap.get(4).unwrap().as_str(),
                request: cap.get(5).unwrap().as_str(),
                response_code: cap[6].parse::<u16>().unwrap(),
                size: cap[7].parse::<u32>().unwrap(),
            })
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ipv6() {
        let input = r#"2600:1700:2825:0:3848:6646:77d4:a280 - - [19/Dec/2020:02:05:35 +0000] "GET / HTTP/1.1" 200 11491
        "#;
        let labels = CommonLogParser::new(input)
            .next()
            .expect("should parse input");
        assert_eq!(labels.ip, "2600:1700:2825:0:3848:6646:77d4:a280")
    }

    #[test]
    fn test_frank() {
        let input = r#"18.141.77.170 - aJVUFc7x [19/Dec/2020:11:13:24 +0000] "GET /manager/html/ HTTP/1.1" 500 8754
        "#;
        let labels = CommonLogParser::new(input)
            .next()
            .expect("should parse input");
        assert_eq!(labels.frank, "aJVUFc7x")
    }

    #[test]
    fn test_all() {
        let input = r#"54.36.148.15 - - [19/Dec/2020:02:04:07 +0000] "GET /a/563915221/alternative-to-miami-radio-live.html HTTP/1.1" 200 10531
        "#;
        let labels = CommonLogParser::new(input)
            .next()
            .expect("should parse input");
        assert_eq!(labels.ip, "54.36.148.15")
    }

    #[test]
    fn test_iter() {
        let input = r#"171.247.180.164 - - [19/Dec/2020:02:04:07 +0000] "GET /a/995540551/alternative-to-king365-choi-game-danh-bai-online.html HTTP/1.1" 200 10994
54.36.148.15 - - [19/Dec/2020:02:04:07 +0000] "GET /a/563915221/alternative-to-miami-radio-live.html HTTP/1.1" 200 10531
45.77.209.201 - - [19/Dec/2020:02:04:09 +0000] broken 
"#;
        let p = CommonLogParser::new(input);

        assert_eq!(p.count(), 2)
    }
}
