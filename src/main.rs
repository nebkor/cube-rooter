use clap::{App, Arg, ArgMatches};

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

const VERSION: &str = "1";

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
                .possible_values(&["exact", "approx", "simd"]),
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

fn get_rooter(args: &ArgMatches) -> Box<dyn Fn(f64) -> f64> {
    match args.value_of("variant").unwrap() {
        _ => Box::new(move |x| x.cbrt()),
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
