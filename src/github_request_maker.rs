use crate::github_url_builder::GitHubUrlBuilder;
use crate::{error_box::ErrorBox, requester::Requester};

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

	fn get_tree_id(&self) -> Result<String, ErrorBox> {
		let json = self.get(
			GitHubUrlBuilder::new()
				.with_repo()
				.with_path("commits")
				.with_query("per_page", 1),
		)?;

		let sha = json[0]["commit"]["tree"]["sha"]
			.as_str()
			.expect("wtf")
			.to_string();
		Ok(sha)
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

	const ERROR_MESSAGE: &str = "Problem making the request";

	#[test]
	fn given_token_expect_authenticated() {
		let token = AuthToken::new("token.txt");
		let requester = RequestMaker::new(Some(token));
		let request_maker = GitHubRequestMaker::new(requester);

		let is_authenticated = request_maker.is_authenticated().expect(ERROR_MESSAGE);
		assert!(is_authenticated);
	}

	#[test]
	fn can_get_tree_id() {
		let request_maker = GitHubRequestMaker::new(TestRequestMaker::new());

		let tree_id = request_maker.get_tree_id().expect(ERROR_MESSAGE);
		assert_eq!(tree_id, "9431e108b67d1efa9df54e6351da1951bcd9be32");
	}

	#[test]
	fn can_get_file_names() {
		let request_maker = GitHubRequestMaker::new(TestRequestMaker::new());
		let file_names = request_maker.get_file_names().expect(ERROR_MESSAGE);

		assert_eq!(
			file_names,
			vec!["yeet.gitignore", "yoink.gitignore", "quite.gitignore"]
		);
	}
}
