use crate::auth_token::AuthToken;
use crate::request_maker::RequestMaker;

type ErrorBox = Box<dyn std::error::Error>;

#[derive(Default)]
pub struct GitHubRequestMaker {
	request_maker: RequestMaker,
}

#[allow(dead_code)] // TODO: REMOVE WHEN CODE IS CALLED IN MAIN!!!!!!!!!
impl GitHubRequestMaker {
	pub fn new() -> GitHubRequestMaker {
		let token = AuthToken::new("token.txt");

		GitHubRequestMaker {
			request_maker: RequestMaker::new(Some(token)),
		}
	}

	pub fn get_latest_commit_id(&self) -> Result<String, ErrorBox> {
		let json = self
			.request_maker
			.get_json("https://api.github.com/repos/github/gitignore/commits?per_page=1")?;

		let sha = json[0]["sha"].as_str().expect("wtf").to_string();
		Ok(sha)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	const ERROR_MESSAGE: &str = "Problem making the request";

	#[test]
	fn can_get_latest_commit_id() {
		let request_maker = GitHubRequestMaker::new();

		let latest_commit_id = request_maker.get_latest_commit_id().expect(ERROR_MESSAGE);

		assert_eq!(latest_commit_id, "80587386dd48334c304819abcc4a09877cf99e21");
	}
}
