use clap::{App, Arg};
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

// usize: 符号なし整数型
#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("headr")
        .version("0.1.0")
        .author("hoge")
        .about("rust head command")
        .get_matches();

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap_or_default(),
        lines: matches.value_of("lines").unwrap_or("10").parse()?,
        bytes: matches.value_of("bytes").map(|b| b.parse()).transpose()?
    })
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:#?}", config);
    Ok(()) 
}

/**
 * コマンドライン引数の文字列を正の整数に変換する
 */
fn parse_positive_int(val: &str) -> MyResult<usize> {

    // 0より大きい整数に変換できる場合はOk、それ以外はErr
    match val.parse() {
        Ok(n) if n > 0 => Ok(n),
        _ => Err(From::from(val))
    }
}

#[test]
fn test_parse_positive_int() {
    // 3は正の整数なのでOK
    let res = parse_positive_int("3");
    assert!(res.is_ok());

    assert_eq!(res.unwrap(), 3);

    // 数字でない文字列の場合はエラー
    let res = parse_positive_int("foo");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "foo".to_string());

    // 0の場合もエラー
    let res = parse_positive_int("0");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "0".to_string());
}