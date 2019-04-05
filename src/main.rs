#[macro_use]
extern crate clap;
use clap::{App, Arg};
use std::mem;

fn q_rsqrt(number: f32) -> f32 {
    let x2 = number * 0.5f32;
    let threehalfs = 1.5f32;

    let mut y = number;
    let mut i: u32 = unsafe { mem::transmute(y) };
    i = 0x5f3759df - (i >> 1);
    y = unsafe { mem::transmute(i) };
    y *= threehalfs - (x2 * y * y);
    y
}

fn main() {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::with_name("input")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .get_matches();

    let input = matches.value_of("input").unwrap();
    let parsed_input = input.parse::<f32>();

    if let Ok(i) = parsed_input {
        println!(
            "The fast inverse square root of {} is approximately {}.",
            i,
            q_rsqrt(i)
        )
    }
}
