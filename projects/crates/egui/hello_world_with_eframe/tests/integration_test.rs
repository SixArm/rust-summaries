#[test]
fn cargo_run() {
    let output = ::std::process::Command::new("cargo").arg("run").output().expect("failure");
    assert!(output.status.success());
    let act = String::from("Hello, World!\nname: age:");
    let exp = String::from_utf8(output.stdout).unwrap();
    assert_eq!(act, exp);
}
