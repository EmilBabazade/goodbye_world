use assert_cmd::Command;

#[test]
fn works() {
    let mut command = Command::cargo_bin("goodbye_world").unwrap();
    command.assert().success().stdout("Goodbye, world!\n");
}
