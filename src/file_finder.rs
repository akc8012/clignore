pub struct FileFinder;

#[allow(dead_code)] // TODO: REMOVE ME WHEN I GET USED
impl FileFinder {
	pub fn find(file_names: Vec<&str>, query: &str) -> Vec<String> {
		let mut matches = Vec::new();
		for file_name in file_names {
			if file_name.to_lowercase().contains(&query.to_lowercase()) {
				matches.push(String::from(file_name));
			}
		}
		matches
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn can_find_file() {
		let file_names = vec!["yeet.gitignore", "yoink.gitignore", "quite.gitignore"];
		let result = &FileFinder::find(file_names, "yeet")[0];
		assert_eq!(result, "yeet.gitignore");
	}

	#[test]
	fn can_find_file_case_insensitive() {
		let file_names = vec!["yeet.gitignore", "yoink.gitignore", "quite.gitignore"];
		let result = &FileFinder::find(file_names, "YeEt")[0];
		assert_eq!(result, "yeet.gitignore");
	}

	#[test]
	fn can_find_no_files() {
		let file_names = vec!["cheese", "pizza"];
		let result = FileFinder::find(file_names, "quite");
		assert_eq!(result.len(), 0);
	}

	#[test]
	fn can_find_multiple_files() {
		let file_names = vec!["funko.gitignore", "lumpo.gitignore", "dunko.gitignore"];
		let result = FileFinder::find(file_names, "unko");
		assert_eq!(result, vec!["funko.gitignore", "dunko.gitignore"]);
	}
}
