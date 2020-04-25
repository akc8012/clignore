use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn can_run_list() {
	let (dir, _token) = helpers::create_temp_dir_with_token();
	let mut cmd = helpers::create_cmd_at_dir(&dir);

	cmd.arg("list");
	cmd.assert()
		.success()
		.stdout(predicate::str::contains("Rust.gitignore"))
		.stdout(predicate::str::contains("Python.gitignore"))
		.stdout(predicate::str::contains("Godot.gitignore"));
}

#[test]
fn can_run_find_single_match() {
	let (dir, _token) = helpers::create_temp_dir_with_token();
	let mut cmd = helpers::create_cmd_at_dir(&dir);

	cmd.arg("find").arg("rust");
	cmd.assert()
		.success()
		.stdout(predicate::str::contains("Downloaded 'Rust.gitignore'"));
}

#[test]
fn can_run_find_single_incomplete_match() {
	let (dir, _token) = helpers::create_temp_dir_with_token();
	let mut cmd = helpers::create_cmd_at_dir(&dir);

	cmd.arg("find").arg("godo");
	cmd.assert()
		.success()
		.stdout(predicate::str::contains("Downloaded 'Godot.gitignore'"));
}

#[test]
fn can_run_find_multiple_matches() {
	let (dir, _token) = helpers::create_temp_dir_with_token();
	let mut cmd = helpers::create_cmd_at_dir(&dir);

	cmd.arg("find").arg("python").write_stdin("1");
	cmd.assert()
		.success()
		.stdout(predicate::str::contains("[1] Python.gitignore"))
		.stdout(predicate::str::contains("Downloaded 'Python.gitignore'"));
}

mod helpers {
	use super::*;

	use std::fs::File;
	use std::io::{Read, Write};
	use tempfile::{Builder, NamedTempFile, TempDir};

	pub fn create_cmd_at_dir(dir: &TempDir) -> Command {
		let mut cmd = Command::cargo_bin("ignore_cli").unwrap();
		cmd.current_dir(dir);
		cmd
	}

	pub fn create_temp_dir_with_token() -> (TempDir, NamedTempFile) {
		let dir = TempDir::new().unwrap();

		// TODO: This should no longer be necessary once the token is set as an ENV variable
		let mut file = Builder::new()
			.prefix("token")
			.suffix(".txt")
			.rand_bytes(0)
			.tempfile_in(&dir)
			.unwrap();

		write!(file, "{}", read_token_from_file("token.txt")).unwrap();
		(dir, file)
	}

	fn read_token_from_file(path: &str) -> String {
		let mut token = String::new();
		File::open(path)
			.unwrap()
			.read_to_string(&mut token)
			.unwrap();

		token
	}
}
