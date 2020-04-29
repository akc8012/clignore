use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn assert_these_tests_are_authenticated() {
	let dir = helpers::create_temp_dir();
	let mut cmd = helpers::create_cmd(&dir);

	cmd.arg("authenticated");
	cmd.assert().success().stdout(predicate::str::contains(
		"According to the GitHub API, you are authenticated!",
	));
}

#[test]
fn can_run_list() {
	let dir = helpers::create_temp_dir();
	let mut cmd = helpers::create_cmd(&dir);

	cmd.arg("list");
	cmd.assert()
		.success()
		.stdout(predicate::str::contains("Rust.gitignore"))
		.stdout(predicate::str::contains("Python.gitignore"))
		.stdout(predicate::str::contains("Godot.gitignore"));

	let gitignore = helpers::get_gitignore(&dir);
	assert_eq!(gitignore, None);
}

#[test]
fn can_run_find_single_match() {
	let dir = helpers::create_temp_dir();
	let mut cmd = helpers::create_cmd(&dir);

	cmd.arg("find").arg("rust");
	cmd.assert()
		.success()
		.stdout(predicate::str::contains("Downloaded .gitignore for 'Rust'"));

	let gitignore = helpers::get_gitignore(&dir).unwrap();
	assert!(predicate::str::contains("/target/").eval(&gitignore));
}

#[test]
fn can_run_find_single_incomplete_match() {
	let dir = helpers::create_temp_dir();
	let mut cmd = helpers::create_cmd(&dir);

	cmd.arg("find").arg("godo");
	cmd.assert()
		.success()
		.stdout(predicate::str::contains("Downloaded .gitignore for 'Godot'"));

	let gitignore = helpers::get_gitignore(&dir).unwrap();
	assert!(predicate::str::contains("export.cfg").eval(&gitignore));
}

#[test]
fn can_run_find_multiple_matches() {
	let dir = helpers::create_temp_dir();
	let mut cmd = helpers::create_cmd(&dir);

	cmd.arg("find").arg("python").write_stdin("1");
	cmd.assert()
		.success()
		.stdout(predicate::str::contains("[1] Python.gitignore"))
		.stdout(predicate::str::contains("Downloaded .gitignore for 'Python'"));

	let gitignore = helpers::get_gitignore(&dir).unwrap();
	assert!(predicate::str::contains(".Python").eval(&gitignore));
}

#[test]
fn can_run_find_and_quit() {
	let dir = helpers::create_temp_dir();
	let mut cmd = helpers::create_cmd(&dir);

	cmd.arg("find").arg("python").write_stdin("0");
	cmd.assert().success();

	let gitignore = helpers::get_gitignore(&dir);
	assert_eq!(gitignore, None);
}

#[test]
fn can_run_find_no_results() {
	let dir = helpers::create_temp_dir();
	let mut cmd = helpers::create_cmd(&dir);

	cmd.arg("find").arg("dank");
	cmd.assert()
		.success()
		.stdout(predicate::str::contains("No matches found for 'dank'"));

	let gitignore = helpers::get_gitignore(&dir);
	assert_eq!(gitignore, None);
}

// TODO: make a test to verify looping logic on invalid input
mod helpers {
	use super::*;

	use std::error::Error;
	use std::fs::File;
	use std::io::Read;
	use tempfile::TempDir;

	pub fn create_cmd(dir: &TempDir) -> Command {
		let mut cmd = Command::cargo_bin("ignore_cli").unwrap();
		cmd.current_dir(dir);

		match read_from_file("token.txt") {
			Ok(token) => {
				cmd.env("TOKEN", token);
			}
			Err(e) => println!("WARNING: No token.txt file!!!!! {}", e),
		};
		cmd
	}

	pub fn create_temp_dir() -> TempDir {
		TempDir::new().unwrap()
	}

	pub fn get_gitignore(dir: &TempDir) -> Option<String> {
		match read_from_file(&format!("{}/.gitignore", dir.path().to_str()?)) {
			Ok(gitignore) => Some(gitignore),
			Err(_) => None,
		}
	}

	fn read_from_file(path: &str) -> Result<String, Box<dyn Error>> {
		let mut token = String::new();
		File::open(path)?.read_to_string(&mut token)?;

		Ok(token)
	}
}
