use std::error::Error;
use clap::{App, Arg};
use std::io::{self, BufRead, BufReader};
use std::fs::File;

/**
 * deriveマクロで、Debugトレイトを追加して構造体を表示できるようにする
 * 
 * Config構造体
 */
#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool, // 行番号を表示するかどうか
    number_nonblank_lines: bool, // 空行以外の行番号を表示するかどうか
}

type MyResult<T> = Result<T, Box<dyn Error>>;

/**
 * 引数としてConfigを受け取る
 */
pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        // 指定したファイルをオープンする
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(_) => println!("Opened {}", filename),
        }
    }
    Ok(())
}

/**
 * 引数を解析し、Config構造体を返す
 */
pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
        .version("0.1.0")
        .author("Hoge <hoge@example.com>")
        .about("Rust cat")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file(s)")
                .multiple(true)
                .default_value("-"),
        )
        .arg(
            Arg::with_name("number")
                .short("n")
                .long("number")
                .takes_value(false)
                .help("Number all output lines"),
        )
        .get_matches();
    
    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(), // 引数の値がUTF-8として解釈させるために values_of_lossy を使う
        number_lines: matches.is_present("number"),
        number_nonblank_lines: matches.is_present("number_nonblank"),
    })
}

/**
 * 指定したファイル名のファイルを開いて、中身を表示する
 * ファイル名が "-" の場合は標準入力から読み込む
 * それ以外の場合は引数に指定したファイルを開く
 */
fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}