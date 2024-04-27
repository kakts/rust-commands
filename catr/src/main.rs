fn main() {
    // catr::get_args()の戻り値がResult型なので、エラーがあればエラーメッセージを表示して終了する
    if let Err(e) = catr::get_args().and_then(catr::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    } 
}
