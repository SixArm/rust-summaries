#[test]
fn cargo_run() {
    let output = ::std::process::Command::new("cargo").arg("run").output().expect("failure");
    assert!(output.status.success());
    let act = String::from_utf8(output.stdout).unwrap();
    let exp = String::from("StringRecord([\"delta\", \"echo\", \"foxtrot\"])\nStringRecord([\"golf\", \"hotel\", \"indigo\"])\n");
    assert_eq!(act, exp);
}
