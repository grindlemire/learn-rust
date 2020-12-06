use std::{error::Error, thread::JoinHandle};

use clap::Clap;
use simple_logger::SimpleLogger;

#[derive(Clap, Debug)]
#[clap(
    name = "service-example",
    about = "An example rust service for running multiple threads."
)]
struct Opts {
    #[clap(
        short,
        long,
        default_value = "5",
        about = "Number of workers that will be running"
    )]
    num_workers: i32,

    #[clap(short, long, parse(from_occurrences), about = "verbosity to log")]
    verbose: i32,
}

fn main() {
    SimpleLogger::new().init().unwrap();
    let opts = Opts::parse();

    log::info!("{:#?}", opts);

    let mut threads = Vec::new();
    for (i, _) in (0..opts.num_workers).enumerate() {
        threads.push(std::thread::spawn(move || log::info!("hello from {}", i)));
    }

    for t in threads {
        t.join().expect("thread failed")
    }
}

trait Runner<T> {
    fn run(&mut self) -> std::io::Result<T>;
    fn close(&mut self) -> std::io::Result<T>;
}

struct Worker {}

impl Worker {
    fn new() -> Worker {
        Worker {}
    }
}

impl Worker for Runner {}
