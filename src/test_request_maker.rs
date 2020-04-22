use crate::{error_box::ErrorBox, requester::Requester};
use serde::de::DeserializeOwned;

pub struct TestRequestMaker;

impl Requester for TestRequestMaker {
	fn get(&self, _url: &str) -> Result<String, ErrorBox> {
		unimplemented!();
	}

	fn get_json(&self, url: &str) -> Result<serde_json::Value, ErrorBox> {
		self.match_github_url(url)
	}

	fn get_json_deserialized<T: DeserializeOwned>(&self, _url: &str) -> Result<T, ErrorBox> {
		unimplemented!();
	}
}

impl TestRequestMaker {
	#[cfg(test)]
	pub fn new() -> TestRequestMaker {
		TestRequestMaker {}
	}

	fn match_github_url(&self, url: &str) -> Result<serde_json::Value, ErrorBox> {
		match url {
			"https://api.github.com/repos/github/gitignore/commits?per_page=1" => 
				Ok(self.commits_json()),
			"https://api.github.com/repos/github/gitignore/git/trees/9431e108b67d1efa9df54e6351da1951bcd9be32?recursive=true" => 
				Ok(self.tree_json()),
			"https://api.github.com/repos/github/gitignore/contents/dank.gitignore" =>
				Ok(self.file_json()),
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
				{ "path": "yeet.gitignore", "type": "blob" },
				{ "path": "yoink.gitignore", "type": "blob" },
				{ "path": "quite.gitignore", "type": "blob" },
				{ "path": "PathToDarkness", "type": "tree" },
			] }
		)
	}

	fn file_json(&self) -> serde_json::Value {
		serde_json::json!({ "content": "LmlkZWE=" })
	}
}
