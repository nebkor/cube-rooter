use clap::{App, Arg, ArgMatches};

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

const VERSION: &str = "1";

fn get_args() -> ArgMatches<'static> {
    App::new("Cube Rooter")
        .version(VERSION)
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

fn get_vals(args: &ArgMatches) -> Vec<f64> {
    let vals_name: &str = args.value_of("NUMS_FILE").unwrap();

    let path = Path::new(vals_name);

    let f = File::open(&path).unwrap_or_else(|_| panic!("Could not open {:#?}", &path));

    let buf_reader = &mut BufReader::new(&f);
    let vals: Vec<f64> = buf_reader
        .lines()
        .map(|x| x.unwrap().parse::<f64>())
        .filter_map(Result::ok).collect();
    vals
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

    for v in vals.into_iter() {
        println!("{} {}", v, rooter(v));
    }
}
