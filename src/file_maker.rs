use std::{fs, io};

struct FileMaker;

#[allow(dead_code)] // TODO: yeet me into the sun
impl FileMaker {
	pub fn make_file(name: &str, content: &str) -> io::Result<()> {
		fs::write(name, content)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn can_make_file() {
		let result = FileMaker::make_file("test_file.txt", "i am the test");
		assert!(result.is_ok());
	}
}
