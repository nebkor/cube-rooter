use clap::{App, Arg};

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn get_vals() -> Vec<f64> {
    let args = App::new("Cube Rooter")
        .arg(
            Arg::with_name("VALS")
                .help("File to read that contains values to cube-root.")
                .required(true)
                .index(1),
        )
        .get_matches();

    let vals_name: String = args.value_of("VALS").unwrap().into();

    let path = Path::new(&vals_name);

    let f = match File::open(&path) {
        Ok(f) => f,
        Err(e) => panic!(format!(
            "could not open {:?} for reading, got {:?}",
            &path, e
        )),
    };

    let ref mut buf_reader = BufReader::new(&f);
    let vals: Vec<f64> = buf_reader
        .lines()
        .map(|x| x.unwrap_or("0.0".to_string()).parse::<f64>().unwrap())
        .collect();
    vals
}

fn main() {
    let vals = get_vals();
    println!("reading from {:?}", vals)
}
