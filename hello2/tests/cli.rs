use assert_cmd::Command;

#[test]
fn runs(){
    let mut cmd = Command::cargo_bin("hello2").unwrap();
    cmd.assert().success();
}