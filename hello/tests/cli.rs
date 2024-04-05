use assert_cmd::Command;

#[test]
fn runs() {

    // 自分のクレート内にあるバイナリを実行する
    // unwrapは、 Result型の中身がOkの場合は中身を取り出し、Errの場合はpanic!する
    let mut cmd = Command::cargo_bin("hello").unwrap();

    cmd.assert().success();
}

