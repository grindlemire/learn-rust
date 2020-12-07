// use std::{error::Error, io::Result, thread::JoinHandle};
use clap::Clap;
use log::info;
use simple_logger::SimpleLogger;
use uuid::Uuid;

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

    info!("{:#?}", opts);

    let mut d = Death::new();

    for _ in 0..20 {
        let worker = Worker::new();
        worker.run();
        d.new_life(worker);
    }

    info!("DEATH: {:?}", d);
    // let mut threads = Vec::new();
    // for (i, _) in (0..opts.num_workers).enumerate() {
    //     threads.push(std::thread::spawn(move || log::info!("hello from {}", i)));
    // }

    // for t in threads {
    //     t.join().expect("thread failed")
    // }
}

trait Runner: std::fmt::Debug {
    fn run(&self) -> ();
    fn close(&self) -> std::io::Error;
}

#[derive(Debug)]
struct Worker {
    id: String,
}

impl Worker {
    fn new() -> Worker {
        Worker {
            id: Uuid::new_v4().to_string(),
        }
    }
}

impl Runner for Worker {
    fn run(&self) {
        info!("Running worker...");
    }

    fn close(&self) -> std::io::Error {
        info!("Closing worker...");
        std::io::Error::new(std::io::ErrorKind::Other, "an error. Oh noes!")
    }
}

#[derive(Debug)]
struct Death {
    lives: Vec<Box<dyn Runner>>,
}

impl Death {
    fn new() -> Death {
        Death { lives: Vec::new() }
    }

    fn new_life<T: Runner + 'static>(&mut self, runner: T) -> &Death {
        self.lives.push(Box::new(runner));
        self
    }
}

trait TNode {
    fn blah(&self);
}
