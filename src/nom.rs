extern crate nom;

use nom::{
    bytes::streaming::{take, take_till, take_while1},
    character::streaming::{char, digit1, space1},
    combinator::{map, map_res, recognize},
    sequence::{preceded, terminated, tuple},
    IResult,
};

pub struct Labels {
    ip: String,
    user: String,
    frank: String,
    date_time: String,
    request: String,
    response_code: u16,
    size: u32,
}

fn is_digit(c: char) -> bool {
    match c {
        '0'..='9' => true,
        _ => false,
    }
}

fn ip(i: &str) -> IResult<&str, String> {
    map(take_while1(|c| is_digit(c) || c == '.'), |s: &str| {
        s.to_string()
    })(i)
}

fn user(i: &str) -> IResult<&str, String> {
    // TODO: support other user identifiers
    map(char('-'), |c| c.to_string())(i)
}

fn frank(i: &str) -> IResult<&str, String> {
    // TODO: support other user identifiers
    map(char('-'), |c| c.to_string())(i)
}

fn date_time(i: &str) -> IResult<&str, String> {
    map(
        preceded(char('['), terminated(take_till(|c| c == ']'), char(']'))),
        |s: &str| s.to_string(),
    )(i)
}

fn request(i: &str) -> IResult<&str, String> {
    map(
        preceded(char('"'), terminated(take_till(|c| c == '"'), char('"'))),
        |s: &str| s.to_string(),
    )(i)
}

fn response_code(i: &str) -> IResult<&str, u16> {
    map_res(take(3usize), |s: &str| s.parse::<u16>())(i)
}

fn size(i: &str) -> IResult<&str, u32> {
    map_res(recognize(digit1), |s: &str| s.parse::<u32>())(i)
}

pub fn parse(i: &str) -> IResult<&str, Labels> {
    let (input, (ip, _, user, _, frank, _, date_time, _, request, _, response_code, _, size)) =
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all() {
        let input = r#"54.36.148.15 - - [19/Dec/2020:02:04:07 +0000] "GET /a/563915221/alternative-to-miami-radio-live.html HTTP/1.1" 200 10531\n"#;
        let (_, labels) = parse(input).expect("should parse correctly");
        assert_eq!(labels.ip, "54.36.148.15")
    }
}
