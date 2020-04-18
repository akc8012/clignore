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

	pub fn with_path(mut self, path: &str) -> GitHubUrlBuilder {
		self.url.push_str(&format!("/{}", path));
		self
	}

	pub fn build(&self) -> String {
		self.url.clone()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn can_build_rate_limit_url() {
		let url = GitHubUrlBuilder::new().with_path("rate_limit").build();
		assert_eq!(url, "https://api.github.com/rate_limit");
	}

	// https://api.github.com/rate_limit
	// https://api.github.com/repos/github/gitignore/commits?per_page=1
	// https://api.github.com/repos/github/gitignore/git/trees/{tree_sha}?recursive=true
}
