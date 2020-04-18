struct GitHubUrlBuilder {
	url: String,
}

#[allow(dead_code)] // TODO: REMOVE WHEN CODE IS CALLED IN MAIN!!!!!!!!!
impl GitHubUrlBuilder {
	pub fn new() -> GitHubUrlBuilder {
		GitHubUrlBuilder {
			url: String::from("https://api.github.com"),
		}
	}

	pub fn with_repo(self) -> GitHubUrlBuilder {
		self.with_path("repos/github/gitignore")
	}

	pub fn with_path(mut self, path: &str) -> GitHubUrlBuilder {
		self.url.push_str(&format!("/{}", path));
		self
	}

	pub fn with_query<T: std::fmt::Display>(mut self, name: &str, value: T) -> GitHubUrlBuilder {
		self.url.push_str(&format!("?{}={}", name, value));
		self
	}

	pub fn build(self) -> String {
		self.url
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn can_build_rate_limit_url() {
		let url = GitHubUrlBuilder::new().with_path("rate_limit").build();
		let expected = "https://api.github.com/rate_limit";

		assert_eq!(url, expected);
	}

	#[test]
	fn can_build_repo_url() {
		let url = GitHubUrlBuilder::new().with_repo().build();
		let expected = "https://api.github.com/repos/github/gitignore";

		assert_eq!(url, expected);
	}

	#[test]
	fn can_build_query_string_url() {
		let url = GitHubUrlBuilder::new()
			.with_repo()
			.with_path("commits")
			.with_query("per_page", 1)
			.build();

		let expected = "https://api.github.com/repos/github/gitignore/commits?per_page=1";
		assert_eq!(url, expected);
	}

	#[test]
	fn can_build_tree_url() {
		let url = GitHubUrlBuilder::new()
			.with_repo()
			.with_path("git/trees/abcdefg")
			.with_query("recursive", true)
			.build();

		let expected =
			"https://api.github.com/repos/github/gitignore/git/trees/abcdefg?recursive=true";
		assert_eq!(url, expected);
	}
}
