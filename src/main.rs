use clap::{App, Arg, ArgMatches};

use std::f64;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

const VERSION: &str = "1";

// see http://h14s.p5r.org/2012/09/0x5f3759df.html
// The integer representation of a float transformed
// by a fractional exponential function is:
//
// Iy = (1 - p) * L * (B - o) + p * Ix
//
// p = exPonent = 0.3333333 for cube root
// L = 2^(mantissa bits) = 2^52 for double precision IEEE-754
// B = "bias" in the biased exponent; see https://en.wikipedia.org/wiki/Double-precision_floating-point_format#IEEE_754_double-precision_binary_floating-point_format:_binary64
// o = offset, for shifting the curve up or down
// Ix = integer representation of IEEE 754 float

// MAGIC = (1 - p) * L * (B - o)
// 64-bit: ((2.0 * 4503599627370496.0f64 * (1023.0 - 0.035)) as u64) / 3;
const MAGIC64: u64 = 0x2A9FA06D3A06D3A0;

const ONE_THIRD: f64 = 1.0f64 / 3.0f64;

fn get_args() -> ArgMatches<'static> {
    App::new("Cube Rooter")
        .version(VERSION)
        .about("Reads a newline-separated sequence or stream of numbers from a file, and displays them next to their [positive] cube roots.")
        .arg(
            Arg::with_name("NUMS_FILE")
                .help("File to read that contains numbers to cube-root.")
                .required(true)
                .short("f")
                .long("nums-file")
                .last(false)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("variant")
                .help("Type of cube root function to use.")
                .index(1)
                .required(true)
                .possible_values(&["exact", "approx", "fast-approx", "simd"]),
        )
        .get_matches()
}

fn get_vals(args: &ArgMatches) -> impl Iterator<Item = f64> {
    let vals_name: &str = args.value_of("NUMS_FILE").unwrap();
    let path = Path::new(vals_name);
    let f = File::open(&path).unwrap_or_else(|_| panic!("Could not open {:#?}", &path));
    BufReader::new(f)
        .lines()
        .filter_map(|x| x.unwrap().parse::<f64>().ok())
}

fn approx(x: f64) -> f64 {
    let mut i: u64 = x.to_bits();
    i = (MAGIC64 as f64 + (ONE_THIRD * i as f64)) as u64;
    f64::from_bits(i)
}

fn fast_approx(x: f64) -> f64 {
    let mut i: u64 = x.to_bits();
    // approximately divide by 3, add to MAGIC; see
    // http://www.hackersdelight.org/hdcodetxt/acbrt.c.txt
    i = i / 4 + i / 16;
    i += i / 16;
    i += i / 256;
    i += MAGIC64;
    f64::from_bits(i)
}

fn get_rooter(args: &ArgMatches) -> Box<dyn Fn(f64) -> f64> {
    match args.value_of("variant").unwrap() {
        "exact" => Box::new(|x| x.cbrt()),
        "approx" => Box::new(approx),
        "fast-approx" => Box::new(fast_approx),
        _ => panic!("why did you do this"),
    }
}

fn main() {
    let args = get_args();
    let vals = get_vals(&args);
    let rooter = get_rooter(&args);

    for v in vals {
        println!("{} {}", v, rooter(v));
    }
}
