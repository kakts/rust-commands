use std::error::Error;
use clap::{App, Arg};

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
        println!("{}", filename);
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

