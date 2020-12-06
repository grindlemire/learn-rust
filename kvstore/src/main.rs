use std::env::{self};
use std::{collections::HashMap, io::Result};

fn main() {
    let mut args = env::args().skip(1);
    let key = args.next().expect("Key was missing");
    let value = args.next().expect("Value was missing");

    let mut database = Database::new().expect("Did not create database");
    database.insert(&key, &value);
    database.insert(&key.to_uppercase(), &value);
    match database.flush() {
        Ok(()) => {
            println!("flushed manually");
        }
        Err(error) => {
            println!("Error while flushing manually: {}", error);
        }
    }
}

struct Database {
    map: HashMap<String, String>,
    flushed: bool,
}
impl Database {
    fn new() -> Result<Database> {
        // This code is equivalent to the contents line below
        // let contents = match std::fs::read_to_string("kv.db") {
        //     Ok(c) => c,
        //     Err(error) => return Err(error),
        // };
        let mut map = HashMap::new();
        let contents = std::fs::read_to_string("kv.db")?;
        for line in contents.lines() {
            let mut chunks = line.splitn(2, '\t');
            let key = chunks.next().expect("No key!");
            let value = chunks.next().expect("No value!");
            map.insert(key.to_owned(), value.to_owned());
        }

        Ok(Database {
            map,
            flushed: false,
        })
    }

    fn insert(&mut self, key: &str, value: &str) {
        self.flushed = false;
        self.map.insert(key.to_owned(), value.to_owned());
    }

    fn flush(&mut self) -> Result<()> {
        println!("flushed!");
        let mut contents = String::new();
        for (key, value) in &self.map {
            let kvpair = format!("{}\t{}\n", key, value);
            contents.push_str(&kvpair);
        }
        self.flushed = true;
        return std::fs::write("kv.db", contents);
    }
}

impl Drop for Database {
    fn drop(&mut self) {
        if !self.flushed {
            let _ = self.flush();
        }
    }
}
