pub struct FileFinder;

impl FileFinder {
	pub fn find(file_names: &[String], query: &str) -> Vec<String> {
		let mut results = Vec::new();
		for file_name in file_names {
			if Self::matches(file_name, query) {
				results.push(file_name.into());
			}
		}
		results
	}

	fn matches(file_name: &str, query: &str) -> bool {
		file_name.to_lowercase().contains(&query.to_lowercase())
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn can_find_file() {
		let file_names = vec![
			"yoink.gitignore".into(),
			"yeet.gitignore".into(),
			"quite.gitignore".into(),
		];
		let result = &FileFinder::find(&file_names, "yeet")[0];
		assert_eq!(result, "yeet.gitignore");
	}

	#[test]
	fn can_find_file_case_insensitive() {
		let file_names = vec![
			"yoink.gitignore".into(),
			"yeet.gitignore".into(),
			"quite.gitignore".into(),
		];
		let result = &FileFinder::find(&file_names, "YeEt")[0];
		assert_eq!(result, "yeet.gitignore");
	}

	#[test]
	fn can_find_no_files() {
		let file_names = vec!["cheese".into(), "pizza".into()];
		let result = FileFinder::find(&file_names, "quite");
		assert_eq!(result.len(), 0);
	}

	#[test]
	fn can_find_multiple_files() {
		let file_names = vec![
			"funko.gitignore".into(),
			"lumpo.gitignore".into(),
			"dunko.gitignore".into(),
		];
		let result = FileFinder::find(&file_names, "unko");
		assert_eq!(result, vec!["funko.gitignore", "dunko.gitignore"]);
	}
}
