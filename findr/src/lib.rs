use crate::EntryType::*;
use clap::{App, Arg};
use regex::Regex;
use std::error::Error;
use walkdir::WalkDir;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Eq, PartialEq)]
enum EntryType {
    Dir,
    File,
    Link,
}

#[derive(Debug)]
pub struct Config {
    paths: Vec<String>,
    names: Vec<Regex>,
    entry_types: Vec<EntryType>,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("findr")
        .version("0.1.0")
        .author("hoge")
        .about("Rust find")
        .arg(
            Arg::with_name("paths")
                .value_name("PATH")
                .help("Path to start searching")
                .default_value(".")
                .multiple(true)
        )
        .arg(
            Arg::with_name("names")
                .value_name("NAME")
                .help("Name")
                .long("name")
                .short("n")
                .multiple(true)
                .takes_value(true)
        )
        .arg(
            Arg::with_name("types")
                .value_name("TYPE")
                .short("t")
                .long("type")
                .help("Entry type")
                .possible_values(&["d", "f", "l"])
                .multiple(true)
                .takes_value(true)
        )
        .get_matches();

    let names = matches
        .values_of_lossy("names")
        .map(|values| {
            values.into_iter()
                .map(|v| {
                    Regex::new(&v)
                        .map_err(|_| format!("Invalid --name \"{}\"", v))
                })
                .collect::<Result<Vec<_>, _>>()
        })
        .transpose()?
        .unwrap_or_default();
    let entry_types = matches
        .values_of_lossy("types")
        .map(|values| {
            values.iter()
                .map(|v| {
                    match v.as_str() {
                        "d" => Dir,
                        "f" => File,
                        "l" => Link,
                        _ => unreachable!("Invalid type"),
                    }
                })
                .collect()
        })
        .unwrap_or_default();
    Ok(Config {
        paths: matches
            .values_of_lossy("paths")
            .unwrap_or_default(),
        names,
        entry_types
    })
}

pub fn run(config: Config) -> MyResult<()> {
    for path in config.paths {
        for entry in WalkDir::new(path) {
            match entry {
                Err(e) => eprintln!("{}", e),
                Ok(entry) => println!("{}", entry.path().display()),
            }
        }
    }
    Ok(())
}