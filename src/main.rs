use std::path::PathBuf;
use structopt::StructOpt;
use rand::prelude::*;

#[macro_use]
extern crate log;

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(short = "v", long = "verbose")]
    verbose: bool,

    #[structopt(short = "r", long = "result", parse(from_os_str))]
    result_file: PathBuf,

    #[structopt(name = "FILE", parse(from_os_str))]
    files: Vec<PathBuf>,
}

fn main() {
    let mut rng = thread_rng();
    println!("{}", rng.gen_range(0..20));
    println!("{}", rng.gen::<f64>());
    println!("{}", if rng.gen() {"head"} else {"tail"});

    env_logger::init();
    error!("Error message");
    warn!("Warning message");
    info!("Info message");
    debug!("Debug message");

    println!("{:#?}", Opt::from_args());
}
