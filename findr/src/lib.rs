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

    let type_filter = |entry: &walkdir::DirEntry| {
        config.entry_types.is_empty()
            || config.entry_types.iter().any(|entry_type| {
            match entry_type {
                Dir => entry.file_type().is_dir(),
                File => entry.file_type().is_file(),
                Link => entry.file_type().is_symlink(),
            }
        })
    };

    let name_filter = |entry: &walkdir::DirEntry| {
        config.names.is_empty()
            || config.names.iter().any(|name| {
            name.is_match(&entry.file_name().to_string_lossy())
        })
    };
    for path in config.paths {
        let entries = WalkDir::new(path)
            .into_iter()
            .filter_map(|e| match e {
                Err(e) => {
                    eprintln!("{}", e);
                    None
                }
                Ok(e) => Some(e),
            })
            .filter(type_filter)
            .filter(name_filter)
            .map(|e| e.path().display().to_string())
            .collect::<Vec<_>>();
        println!("{}", entries.join("\n"));
    }
    Ok(())
}