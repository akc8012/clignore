use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn can_run_list() {
	let mut cmd = Command::cargo_bin("ignore_cli").unwrap();
	cmd.arg("list");
	cmd.assert()
		.success()
		.stdout(predicate::str::contains("Rust.gitignore"))
		.stdout(predicate::str::contains("Python.gitignore"))
		.stdout(predicate::str::contains("Godot.gitignore"));
}

#[test]
fn can_run_find_single_match() {
	let mut cmd = Command::cargo_bin("ignore_cli").unwrap();
	cmd.arg("find").arg("rust");
	cmd.assert()
		.success()
		.stdout(predicate::str::contains("Downloaded 'Rust.gitignore'"));
}

#[test]
fn can_run_find_single_incomplete_match() {
	let mut cmd = Command::cargo_bin("ignore_cli").unwrap();
	cmd.arg("find").arg("godo");
	cmd.assert()
		.success()
		.stdout(predicate::str::contains("Downloaded 'Godot.gitignore'"));
}

#[test]
fn can_run_find_multiple_matches() {
	let mut cmd = Command::cargo_bin("ignore_cli").unwrap();
	cmd.arg("find").arg("python").write_stdin("1");
	cmd.assert()
		.success()
		.stdout(predicate::str::contains("[1] Python.gitignore"))
		.stdout(predicate::str::contains("Downloaded 'Python.gitignore'"));
}
