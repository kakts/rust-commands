use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

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
        .arg(
            Arg::with_name("lines")
                .value_name("LINES")
                .short("n")
                .long("lines")
                .help("Number of lines")
                .default_value("10")
        )
        .arg(
            Arg::with_name("bytes")
                .value_name("BYTES")
                .short("c")
                .long("bytes")
                .takes_value(true)
                .help("Number of files")
                .conflicts_with("lines")
        )
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file(s)")
                .multiple(true)
                .default_value("-"),
        )

        .get_matches();

    // 引数のパース
    let lines = matches
        .value_of("lines")
        .map(parse_positive_int)
        .transpose() // Option<Result<T, E>> -> Result<Option<T>, E>
        .map_err(|e| format!("illegal line count -- {}", e))?;

    let bytes = matches
        .value_of("bytes")
        .map(parse_positive_int)
        .transpose()
        .map_err(|e| format!("illegal byte count -- {}", e))?;


    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        lines: lines.unwrap(),
        bytes,
    })
}

pub fn run(config: Config) -> MyResult<()> {

    let num_files = config.files.len();

    for (file_num, filename) in config.files.iter().enumerate() {
        match open(&filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(mut file) => {

                // 複数のファイルが指定されている場合は、ファイル名を表示
                if num_files > 1 {
                    println!(
                        "{}==> {} <==",
                        if file_num > 0 { "\n" } else { "" },
                        filename
                    );
                }


                if let Some(num_bytes) = config.bytes {
                    // 指定したバイト数だけ読み込む
                    let mut handle = file.take(num_bytes as u64);
                    // ファイルから読み込んだバイトを保持するために、0で初期化したnum_bytes長の可変バッファの作成
                    let mut buffer = vec![0; num_bytes];
                    let bytes_read = handle.read(&mut buffer)?;

                    // 実際に読み込まれたバイト数を文字列に変換して出力
                    print!("{}", String::from_utf8_lossy(&buffer[..bytes_read]));
                } else {
                    let mut line = String::new();
                    for _ in 0..config.lines {
                        let bytes = file.read_line(&mut line)?;

                        // EOFに達すると0byteが返される
                        if bytes == 0 {
                            break;
                        }
                        print!("{}", line);
                        // バッファをクリア
                        line.clear();
                    }
                }
            } 
        }
    }

    Ok(()) 
}

/**
 * コマンドライン引数の文字列を正の整数に変換する
 */
fn parse_positive_int(val: &str) -> MyResult<usize> {

    // 0より大きい整数に変換できる場合はOk、それ以外はErr
    match val.parse() {
        Ok(n) if n > 0 => Ok(n),
        _ => Err(From::from(val)) // 数値に変換できない場合はエラー
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

/**
 * ファイルopen
 */
fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(std::io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}