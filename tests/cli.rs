use assert_cmd::Command;

#[test]
fn working() {}

#[test]
fn no_arguments() {
    let mut cmd = Command::cargo_bin("ece461_team19_cli").unwrap();
    cmd.assert().failure();
}

#[test]
fn bad_arguments() {
    let mut cmd1 = Command::cargo_bin("ece461_team19_cli").unwrap();
    cmd1.arg("what").assert().failure();

    let mut cmd2 = Command::cargo_bin("ece461_team19_cli").unwrap();
    cmd2.arg("url").assert().failure();

    let mut cmd3 = Command::cargo_bin("ece461_team19_cli").unwrap();
    cmd3.arg("report").assert().failure();
}

#[test]
fn bad_file_name() {
    let mut cmd = Command::cargo_bin("ece461_team19_cli").unwrap();
    cmd.args(["url", "notafile.osu"]).assert().failure();
}

#[test]
fn empty_file() {
    let mut cmd = Command::cargo_bin("ece461_team19_cli").unwrap();
    cmd.args(["url", "tests/empty.txt"])
        .assert()
        .success()
        .stdout("");
}

#[test]
fn new_lines() {
    let mut cmd = Command::cargo_bin("ece461_team19_cli").unwrap();
    cmd.args(["url", "tests/newline.txt"])
        .assert()
        .success()
        .stdout("");
}

#[test]
fn bad_url() {
    let mut cmd = Command::cargo_bin("ece461_team19_cli").unwrap();
    cmd.args(["url", "tests/badurl.txt"]).assert().failure();
}

#[test]
fn good_urls() {
    let mut cmd = Command::cargo_bin("ece461_team19_cli").unwrap();
    cmd.args(["url", "tests/url.txt"]).assert().success();
}
