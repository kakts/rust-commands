use clap::{App, Arg};

fn main() {
    // echorプログラムを -h か --help でヘルプを表示するようにする
    let matches = App::new("echor")
        .version("0.1.0") // -v か --version でバージョンを表示するようにする
        .author("Ken Youens-Clark <hoge@hoge.com>")
        .about("Rust echo")
        .arg(
            Arg::with_name("text")
                .value_name("TEXT")
                .help("Input text") // 引数の名前
                .required(true) // 必須
                .min_values(1), // 最小値
        )
        .arg(
            Arg::with_name("omit_newline")
                .short("n") // -n で省略
                .help("Do not print newline")
                .takes_value(false), // 値を取らない
        )
        .get_matches(); // 引数を解析するようにappに指示
    
    println!("{:#?}", matches);
}
