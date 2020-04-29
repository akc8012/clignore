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
		let files = json["tree"].as_array().expect("wtf");

		for file in files.iter().filter(|f| f["type"] == "blob") {
			file_names.push(file["path"].as_str().expect("wtf").to_string());
		}

		Ok(file_names)
	}

	pub fn get_file(&self, path: &str) -> Result<String, ErrorBox> {
		let json = self.get(
			GitHubUrlBuilder::new()
				.with_repo()
				.with_path(&format!("contents/{}", path)),
		)?;

		let encoded_file = json["content"].as_str().expect("wtf");
		let encoded_file = encoded_file.replace("\n", "");

		let file = &base64::decode(encoded_file)?;
		Ok(std::str::from_utf8(file)?.to_string())
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

	pub fn is_authenticated(&self) -> Result<bool, ErrorBox> {
		Ok(self.get_rate_limit("limit")? == 5000)
	}

	pub fn too_close_to_limit(&self) -> Result<Option<u16>, ErrorBox> {
		let rate_limit = self.get_rate_limit("remaining")?;

		if rate_limit <= 10 {
			Ok(Some(rate_limit as u16))
		} else {
			Ok(None)
		}
	}

	fn get(&self, url: GitHubUrlBuilder) -> Result<serde_json::Value, ErrorBox> {
		self.request_maker.get_json(&url.build())
	}

	fn get_rate_limit(&self, key: &str) -> Result<u64, ErrorBox> {
		let json = self.get(GitHubUrlBuilder::new().with_path("rate_limit"))?;
		Ok(json["resources"]["core"][key].as_u64().expect("wtf"))
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::auth_token::AuthToken;
	use crate::request_maker::RequestMaker;
	use crate::test_request_maker::TestRequestMaker;

	#[test]
	#[ignore] // requires token.txt file
	fn given_token_expect_authenticated() {
		let token = AuthToken::new("token.txt").unwrap();
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
	fn cannot_get_path_names() {
		let request_maker = GitHubRequestMaker::new(TestRequestMaker::new());
		let file_names = request_maker.get_file_names().unwrap();

		assert!(!file_names.iter().any(|f| f == "PathToDarkness"));
	}

	#[test]
	fn can_download_file() {
		let request_maker = GitHubRequestMaker::new(TestRequestMaker::new());
		let file = request_maker.get_file("dank.gitignore").unwrap();

		assert_eq!(file, ".idea");
	}

	#[test]
	fn can_get_too_close_to_limit() {
		let request_maker = GitHubRequestMaker::new(TestRequestMaker::new());
		let result = request_maker.too_close_to_limit().unwrap();

		match result {
			Some(limit) => assert_eq!(limit, 10),
			None => assert!(false),
		}
	}
}
