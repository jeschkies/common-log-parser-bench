use super::Labels;

use nom::{
    bytes::streaming::{take, take_while, take_while1},
    character::streaming::{char, digit1, newline, space1},
    combinator::{map_res, recognize},
    sequence::{preceded, terminated, tuple},
    AsChar, IResult,
};

fn surrounded<'t>(begin: char, end: char) -> impl FnMut(&'t str) -> IResult<&'t str, &'t str> {
    preceded(
        char(begin),
        terminated(take_while(move |c| c != end), char(end)),
    )
}

fn ip(i: &str) -> IResult<&str, &str> {
    // Parse ipv4 or ipv6
    take_while1(|c: char| c.is_hex_digit() || c == '.' || c == ':')(i)
}

fn user(i: &str) -> IResult<&str, &str> {
    // TODO: support other user identifiers
    recognize(char('-'))(i)
}

fn frank(i: &str) -> IResult<&str, &str> {
    recognize(take_while1(|c| c != ' '))(i)
}

fn date_time(i: &str) -> IResult<&str, &str> {
    surrounded('[', ']')(i)
}

fn request(i: &str) -> IResult<&str, &str> {
    surrounded('"', '"')(i)
}

fn response_code(i: &str) -> IResult<&str, u16> {
    map_res(take(3usize), |s: &str| s.parse::<u16>())(i)
}

fn size(i: &str) -> IResult<&str, u32> {
    map_res(recognize(digit1), |s: &str| s.parse::<u32>())(i)
}

pub fn parse(i: &str) -> IResult<&str, Labels> {
    let (input, (ip, _, user, _, frank, _, date_time, _, request, _, response_code, _, size, _)) =
        tuple((
            ip,
            space1,
            user,
            space1,
            frank,
            space1,
            date_time,
            space1,
            request,
            space1,
            response_code,
            space1,
            size,
            terminated(take_while(|c: char| c != '\n'), newline),
        ))(i)?;

    Ok((
        input,
        Labels {
            ip,
            user,
            frank,
            date_time,
            request,
            response_code,
            size,
        },
    ))
}

#[derive(Clone, Copy)]
pub struct CommonLogParser<'t> {
    pub input: &'t str,
}

impl<'t> Iterator for CommonLogParser<'t> {
    type Item = Labels<'t>;

    fn next(&mut self) -> Option<Labels<'t>> {
        if let Ok((out, labels)) = parse(self.input) {
            self.input = out;
            Some(labels)
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
        let (_, labels) = parse(input).expect("should parse correctly");
        assert_eq!(labels.ip, "2600:1700:2825:0:3848:6646:77d4:a280")
    }

    #[test]
    fn test_frank() {
        let input = r#"18.141.77.170 - aJVUFc7x [19/Dec/2020:11:13:24 +0000] "GET /manager/html/ HTTP/1.1" 500 8754
        "#;
        let (_, labels) = parse(input).expect("should parse correctly");
        assert_eq!(labels.frank, "aJVUFc7x")
    }

    #[test]
    fn test_all() {
        let input = r#"54.36.148.15 - - [19/Dec/2020:02:04:07 +0000] "GET /a/563915221/alternative-to-miami-radio-live.html HTTP/1.1" 200 10531
        "#;
        let (_, labels) = parse(input).expect("should parse correctly");
        assert_eq!(labels.ip, "54.36.148.15");
        assert_eq!(labels.user, "-");
        assert_eq!(labels.frank, "-");
        assert_eq!(labels.date_time, "19/Dec/2020:02:04:07 +0000");
        assert_eq!(labels.request, "GET /a/563915221/alternative-to-miami-radio-live.html HTTP/1.1");
        assert_eq!(labels.response_code, 200);
        assert_eq!(labels.size, 10531);
    }

    #[test]
    fn test_iter() {
        let input = r#"171.247.180.164 - - [19/Dec/2020:02:04:07 +0000] "GET /a/995540551/alternative-to-king365-choi-game-danh-bai-online.html HTTP/1.1" 200 10994
54.36.148.15 - - [19/Dec/2020:02:04:07 +0000] "GET /a/563915221/alternative-to-miami-radio-live.html HTTP/1.1" 200 10531
45.77.209.201 - - [19/Dec/2020:02:04:09 +0000] broken 
"#;
        let p = CommonLogParser { input };

        assert_eq!(p.count(), 2)
    }
}
