use assert_cmd::Command;

#[test]
fn runs() {

    // 自分のクレート内にあるバイナリを実行する
    // unwrapは、 Result型の中身がOkの場合は中身を取り出し、Errの場合はpanic!する
    let mut cmd = Command::cargo_bin("hello").unwrap();

    cmd.assert().success().stdout("Hello, world!\n");
}

// bin/true.rsを実行するテスト
#[test]
fn true_ok() {
    let mut cmd = Command::cargo_bin("true").unwrap();
    cmd.assert().success();
}

// bin/false.rsを実行するテスト
#[test]
fn false_not_ok() {
    let mut cmd = Command::cargo_bin("false").unwrap();
    cmd.assert().failure();
}