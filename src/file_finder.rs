pub struct FileFinder;

#[allow(dead_code)] // TODO: REMOVE ME WHEN I GET USED
impl FileFinder {
	pub fn find(file_names: &Vec<&str>, query: &str) -> Option<String> {
		for file_name in file_names {
			if file_name.contains(query) {
				return Some(file_name.to_string());
			}
		}

		None
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn can_find_file() {
		let file_names = vec!["yeet.gitignore", "yoink.gitignore", "quite.gitignore"];
		let result = FileFinder::find(&file_names, "yeet").unwrap();
		assert_eq!(result, "yeet.gitignore");
	}
}
