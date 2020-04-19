pub struct FileFinder;

impl FileFinder {
	pub fn find<'s>(file_names: &'s [String], query: &str) -> Vec<&'s String> {
		let mut indices = Vec::new();
		for (index, file_name) in file_names.iter().enumerate() {
			if Self::matches(file_name, query) {
				indices.push(index);
			}
		}
		Self::subset(&file_names, &indices)
	}

	fn matches(file_name: &str, query: &str) -> bool {
		file_name.to_lowercase().contains(&query.to_lowercase())
	}

	fn subset<'s>(file_names: &'s [String], indices: &[usize]) -> Vec<&'s String> {
		indices.iter().map(|&i| &file_names[i]).collect()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn can_find_file() {
		let file_names = vec![
			String::from("yoink.gitignore"),
			String::from("yeet.gitignore"),
			String::from("quite.gitignore"),
		];
		let result = FileFinder::find(&file_names, "yeet")[0];
		assert_eq!(result, "yeet.gitignore");
	}

	#[test]
	fn can_find_file_case_insensitive() {
		let file_names = vec![
			String::from("yoink.gitignore"),
			String::from("yeet.gitignore"),
			String::from("quite.gitignore"),
		];
		let result = FileFinder::find(&file_names, "YeEt")[0];
		assert_eq!(result, "yeet.gitignore");
	}

	#[test]
	fn can_find_no_files() {
		let file_names = vec![String::from("cheese"), String::from("pizza")];
		let result = FileFinder::find(&file_names, "quite");
		assert_eq!(result.len(), 0);
	}

	#[test]
	fn can_find_multiple_files() {
		let file_names = vec![
			String::from("funko.gitignore"),
			String::from("lumpo.gitignore"),
			String::from("dunko.gitignore"),
		];
		let result = FileFinder::find(&file_names, "unko");
		assert_eq!(result, vec!["funko.gitignore", "dunko.gitignore"]);
	}
}
