//test duplicates
use assert_cmd::Command;
use predicates::prelude::*;

const PRG: &str = "rdedupe";
const DUPE1: &str = "tests/inputs/one.txt";
const DUPE2: &str = "tests/inputs/same-one.txt";
const NOTDUPE: &str = "tests/inputs/three.txt";

#[test]
fn usage() {
    let mut cmd = Command::cargo_bin(PRG).unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Finds duplicate files"));
}

#[test]
fn search() {
    let mut cmd = Command::cargo_bin(PRG).unwrap();
    cmd.arg("search")
        .arg("--path")
        .arg("tests/inputs")
        .arg("--pattern")
        .arg(".txt")
        .assert()
        .success()
        .stdout(predicate::str::contains(DUPE1))
        .stdout(predicate::str::contains(DUPE2))
        .stdout(predicate::str::contains(NOTDUPE));
}
