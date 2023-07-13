

use clap::{Arg, Command};

fn main() {
    let _matches = Command::new("ml-devtool")
    .version("0.1")
    .author("Kirk Kaiser (kirk@zothcorp.com)")
    .about("Deterministic, reproducible dev environments")
    .arg(Arg::new("container").short('c').long("container"))
    .arg(Arg::new("dockerfile").short('d').long("dockerfile"))
    .get_matches();
}