extern crate clap;

use clap::Clap;

#[derive(Clap)]
#[clap(version = "0.0.2")]
struct Opts {
    #[clap(short, long, default_value = "0")]
    verbose: i32,
}

fn main() {
    let opts = Opts::parse();

    println!("verbose: {}", opts.verbose);
}
