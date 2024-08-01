#[test]
fn cargo_run() {
    let output = ::std::process::Command::new("cargo").arg("run").output().expect("failure");
    assert!(output.status.success());
    let exp = String::from("INFO  [info_warn_error_debug] Example info message");
    let act = String::from_utf8(output.stdout).unwrap().lines().next().unwrap()[25..].to_owned();
    assert_eq!(act, exp);
}
