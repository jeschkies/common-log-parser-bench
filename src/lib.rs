#[macro_use] extern crate lazy_static;

pub mod nom;
pub mod regex;

#[allow(dead_code)]
pub struct Labels<'t> {
    ip: &'t str,
    user: &'t str,
    frank: &'t str,
    date_time: &'t str,
    request: &'t str,
    response_code: u16,
    size: u32,
}
