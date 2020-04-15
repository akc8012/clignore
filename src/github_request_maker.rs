use crate::request_maker::RequestMaker;

type ErrorBox = Box<dyn std::error::Error>;

#[derive(Default)]
pub struct GitHubRequestMaker {
	request_maker: RequestMaker,
}

#[allow(dead_code)] // TODO: REMOVE WHEN CODE IS CALLED IN MAIN!!!!!!!!!
impl GitHubRequestMaker {
	pub fn new() -> GitHubRequestMaker {
		GitHubRequestMaker {
			request_maker: RequestMaker {},
		}
	}

	pub fn make_github_request() -> Result<String, ErrorBox> {
		// TODO: Fix 403 Forbidden
		RequestMaker::get("https://api.github.com/rate_limit")
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	const ERROR_MESSAGE: &str = "Problem making the request";

	#[test]
	fn talk_to_github() {
		let response = GitHubRequestMaker::make_github_request().expect(ERROR_MESSAGE);
		assert!(response.contains("\"rate\": {"), "Should receive JSON");
	}
}
