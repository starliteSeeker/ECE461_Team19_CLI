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
