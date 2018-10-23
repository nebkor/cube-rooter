use clap::{App, Arg};

use rand::prelude::*;
use rand::FromEntropy;

fn main() {
    let mut rng = SmallRng::from_entropy();

    let num_vals: usize = App::new("Random number generator")
        .arg(
            Arg::with_name("NUM_VALS")
                .help(
                    "Number of random numbers between 0.0 and 1.0 to print out. Defaults to 1,000.",
                )
                .required(false)
                .index(1),
        )
        .get_matches()
        .value_of("NUM_VALS")
        .unwrap_or("1000")
        .parse()
        .unwrap_or(1000);

    for _ in 0..num_vals {
        println!("{}", rng.gen::<f64>());
    }
}
