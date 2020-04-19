use crate::requester::{ErrorBox, Requester};
use serde::de::DeserializeOwned;

enum GitHubUrl {
	Commits,
	Tree,
}

pub struct TestRequestMaker;

impl Requester for TestRequestMaker {
	fn get(&self, _url: &str) -> Result<String, ErrorBox> {
		panic!("I have no definition and I must scream");
	}

	fn get_json(&self, url: &str) -> Result<serde_json::Value, ErrorBox> {
		match self.match_github_url(url)? {
			GitHubUrl::Commits => Ok(self.commits_json()),
			GitHubUrl::Tree => Ok(self.tree_json()),
		}
	}

	fn get_json_deserialized<T: DeserializeOwned>(&self, _url: &str) -> Result<T, ErrorBox> {
		panic!("I have no definition and I must scream");
	}
}

#[allow(dead_code)]
impl TestRequestMaker {
	pub fn new() -> TestRequestMaker {
		TestRequestMaker {}
	}

	fn match_github_url(&self, url: &str) -> Result<GitHubUrl, ErrorBox> {
		match url {
			"https://api.github.com/repos/github/gitignore/commits?per_page=1" => 
				Ok(GitHubUrl::Commits),
			"https://api.github.com/repos/github/gitignore/git/trees/9431e108b67d1efa9df54e6351da1951bcd9be32?recursive=true" => 
				Ok(GitHubUrl::Tree),
			_ => Err(format!("Unknown GitHub url: {}", url).into())
		}
	}

	fn commits_json(&self) -> serde_json::Value {
		serde_json::json!(
			[{"commit": { "tree": { "sha": "9431e108b67d1efa9df54e6351da1951bcd9be32" } }}]
		)
	}

	fn tree_json(&self) -> serde_json::Value {
		serde_json::json!(
			{ "tree": [
				{ "path": "yeet.gitignore" },
				{ "path": "yoink.gitignore" },
				{ "path": "quite.gitignore" }
			] }
		)
	}
}
