use crate::requester::Requester;

type ErrorBox = Box<dyn std::error::Error>;

pub struct GitHubRequestMaker<T: Requester> {
	request_maker: T,
}

#[allow(dead_code)] // TODO: REMOVE WHEN CODE IS CALLED IN MAIN!!!!!!!!!
impl<T: Requester> GitHubRequestMaker<T> {
	// TODO: Warn when X-RateLimit-Limit < 5000 (not authenticated)

	pub fn new(request_maker: T) -> GitHubRequestMaker<T> {
		GitHubRequestMaker { request_maker }
	}

	// TODO: Better error handling on expects here
	pub fn get_tree_id(&self) -> Result<String, ErrorBox> {
		let json = self.get("commits?per_page=1")?;

		let sha = json[0]["commit"]["tree"]["sha"]
			.as_str()
			.expect("wtf")
			.to_string();
		Ok(sha)
	}

	pub fn get_file_names(&self, tree_id: &str) -> Result<Vec<String>, ErrorBox> {
		let json = self.get(&format!("git/trees/{}?recursive=true", tree_id))?;

		let mut file_names = Vec::new();
		for file in json["tree"].as_array().expect("wtf") {
			file_names.push(file["path"].as_str().expect("wtf").to_string());
		}

		Ok(file_names)
	}

	fn get(&self, path: &str) -> Result<serde_json::Value, ErrorBox> {
		self.request_maker.get_json(&format!(
			"https://api.github.com/repos/github/gitignore/{}",
			path
		))
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_request_maker::TestRequestMaker;

	const ERROR_MESSAGE: &str = "Problem making the request";

	#[test]
	fn can_get_tree_id() {
		let requester = TestRequestMaker::new();
		let request_maker = GitHubRequestMaker::new(requester);

		let tree_id = request_maker.get_tree_id().expect(ERROR_MESSAGE);
		assert_eq!(tree_id, "9431e108b67d1efa9df54e6351da1951bcd9be32");
	}

	#[test]
	fn can_get_file_names() {
		let requester = TestRequestMaker::new();
		let request_maker = GitHubRequestMaker::new(requester);

		let tree_id = request_maker.get_tree_id().expect(ERROR_MESSAGE);
		let file_names = request_maker.get_file_names(&tree_id).expect(ERROR_MESSAGE);

		assert_eq!(
			file_names,
			vec!["yeet.gitignore", "yoink.gitignore", "quite.gitignore"]
		);
	}
}
