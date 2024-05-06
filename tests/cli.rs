use assert_cmd::Command;

#[test]
fn goodbye_world_works() {
    let mut command = Command::cargo_bin("goodbye_world").unwrap();
    command.assert().success().stdout("Goodbye, world!\n");
}

#[test]
fn error_output_works() {
    let mut command = Command::cargo_bin("error_output").unwrap();
    command.assert().success().stderr("oopsie!\n");
}
