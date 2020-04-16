use crate::response_getter::ResponseGetter;

type ErrorBox = Box<dyn std::error::Error>;

pub struct GitHubRequestMaker<T: ResponseGetter> {
	request_maker: T,
}

#[allow(dead_code)] // TODO: REMOVE WHEN CODE IS CALLED IN MAIN!!!!!!!!!
impl<T: ResponseGetter> GitHubRequestMaker<T> {
	pub fn new(request_maker: T) -> GitHubRequestMaker<T> {
		GitHubRequestMaker { request_maker }
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
	use crate::auth_token::AuthToken;
	use crate::request_maker::RequestMaker;

	const ERROR_MESSAGE: &str = "Problem making the request";

	#[test]
	fn can_get_latest_commit_id() {
		let token = AuthToken::new("token.txt");
		let requester = RequestMaker::new(Some(token));
		let request_maker = GitHubRequestMaker::new(requester);

		let latest_commit_id = request_maker.get_latest_commit_id().expect(ERROR_MESSAGE);

		assert_eq!(latest_commit_id, "80587386dd48334c304819abcc4a09877cf99e21");
	}
}
