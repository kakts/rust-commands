use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

/**
 * テストの結果を返す型
 * 成功時: 常にユニット型を含む
 * 失敗時: std::error::Errorトレイトを実装する型を含む
 * 
 * dynキーワード: 動的ディスパッチを行うトレイトオブジェクトを表す
 * 動的ディスパッチ: 
 * トレイトオブジェクトは、特定のトレイトを実装する任意の型の値を表すことができる。
 * しかし、具体的な型はコンパイル時にはわからないため、どの実装を使用するかは実行時に決定される。
 */
type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn dies_no_args() -> TestResult {
    
    Command::cargo_bin("echor")?
        .assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));

    Ok(())
}

/**
 * テスト用のヘルパー関数
 */
fn run(args: &[&str], expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin("echor")?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

#[test]
fn hello1() -> TestResult {
    run(&["Hello there"], "tests/expected/hello1.txt")
}

#[test]
fn hello2() -> TestResult {
    run(&["Hello", "there"], "tests/expected/hello2.txt")
}

#[test]
fn hello1_no_newline() -> TestResult {
    run(&["Hello", "there", "-n"], "tests/expected/hello1.n.txt")
}

#[test]
fn hello2_no_newline() -> TestResult {
    run(&["-n", "Hello", "there"], "tests/expected/hello2.n.txt")
}