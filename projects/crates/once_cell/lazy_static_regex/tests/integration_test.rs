#[test]
fn cargo_run() {
    let output = ::std::process::Command::new("cargo").arg("run").output().expect("failure");
    assert!(output.status.success());
    let act = String::from_utf8(output.stdout).unwrap().to_owned();
    let exp = String::from("true\n");
    assert_eq!(act, exp);
}
