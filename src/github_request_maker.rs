use crate::github_url_builder::GitHubUrlBuilder;
use crate::{error_box::ErrorBox, requester::Requester};
use base64::decode;

pub struct GitHubRequestMaker<T: Requester> {
	request_maker: T,
}

impl<T: Requester> GitHubRequestMaker<T> {
	pub fn new(request_maker: T) -> GitHubRequestMaker<T> {
		GitHubRequestMaker { request_maker }
	}

	// TODO: Better error handling on expects here
	pub fn get_file_names(&self) -> Result<Vec<String>, ErrorBox> {
		let tree_id = self.get_tree_id()?;
		let json = self.get(
			GitHubUrlBuilder::new()
				.with_repo()
				.with_path(&format!("git/trees/{}", &tree_id))
				.with_query("recursive", true),
		)?;

		let mut file_names = Vec::new();
		for file in json["tree"].as_array().expect("wtf") {
			file_names.push(file["path"].as_str().expect("wtf").to_string());
		}

		Ok(file_names)
	}

	#[allow(dead_code)]
	pub fn get_file(&self, path: &str) -> Result<String, ErrorBox> {
		let json = self.get(
			GitHubUrlBuilder::new()
				.with_repo()
				.with_path(&format!("contents/{}", path)),
		)?;

		let encoded_file = json["content"].as_str().expect("wtf");
		let file = &decode(encoded_file).unwrap();
		Ok(std::str::from_utf8(file).unwrap().to_string())
	}

	fn get_tree_id(&self) -> Result<String, ErrorBox> {
		let json = self.get(
			GitHubUrlBuilder::new()
				.with_repo()
				.with_path("commits")
				.with_query("per_page", 1),
		)?;

		let sha = json[0]["commit"]["tree"]["sha"].as_str().expect("wtf");
		Ok(sha.to_string())
	}

	// TODO: Warn when we get too close to rate limit
	#[allow(dead_code)]
	pub fn is_authenticated(&self) -> Result<bool, ErrorBox> {
		let json = self.get(GitHubUrlBuilder::new().with_path("rate_limit"))?;

		let rate_limit = json["resources"]["core"]["limit"].as_i64().expect("wtf");
		Ok(rate_limit == 5000)
	}

	fn get(&self, url: GitHubUrlBuilder) -> Result<serde_json::Value, ErrorBox> {
		self.request_maker.get_json(&url.build())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::auth_token::AuthToken;
	use crate::request_maker::RequestMaker;
	use crate::test_request_maker::TestRequestMaker;

	#[test]
	fn given_token_expect_authenticated() {
		let token = AuthToken::new("token.txt");
		let requester = RequestMaker::new(Some(token));
		let request_maker = GitHubRequestMaker::new(requester);

		let is_authenticated = request_maker.is_authenticated().unwrap();
		assert!(is_authenticated);
	}

	#[test]
	fn can_get_tree_id() {
		let request_maker = GitHubRequestMaker::new(TestRequestMaker::new());

		let tree_id = request_maker.get_tree_id().unwrap();
		assert_eq!(tree_id, "9431e108b67d1efa9df54e6351da1951bcd9be32");
	}

	#[test]
	fn can_get_file_names() {
		let request_maker = GitHubRequestMaker::new(TestRequestMaker::new());
		let file_names = request_maker.get_file_names().unwrap();

		assert_eq!(
			file_names,
			vec!["yeet.gitignore", "yoink.gitignore", "quite.gitignore"]
		);
	}

	#[test]
	fn can_download_file() {
		let request_maker = GitHubRequestMaker::new(TestRequestMaker::new());
		let file = request_maker.get_file("dank.gitignore").unwrap();

		assert_eq!(file, ".idea");
	}
}
