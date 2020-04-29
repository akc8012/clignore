use crate::{auth_token::AuthToken, error_box::ErrorBox, requester::Requester};
use reqwest::{blocking, header};

#[derive(Default)]
pub struct RequestMaker {
	token: Option<AuthToken>,
}

impl Requester for RequestMaker {
	fn get_json(&self, url: &str) -> Result<serde_json::Value, ErrorBox> {
		let object = self.get_response(url)?.json()?;
		Ok(object)
	}
}

impl RequestMaker {
	pub fn new(token: Option<AuthToken>) -> RequestMaker {
		RequestMaker { token }
	}

	fn get_response(&self, url: &str) -> Result<blocking::Response, ErrorBox> {
		let response = self.create_client().get(url).send()?;

		let status = response.status();
		if status != 200 {
			let body = response.text().unwrap_or_default();
			return Err(format!("Received: {}: {}", status, body).into());
		}

		Ok(response)
	}

	fn create_client(&self) -> blocking::Client {
		let user_agent: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

		blocking::Client::builder()
			.user_agent(user_agent)
			.default_headers(self.get_headers())
			.build()
			.unwrap()
	}

	fn get_headers(&self) -> header::HeaderMap {
		let mut headers = header::HeaderMap::new();

		if let Some(token) = &self.token {
			headers.insert(
				header::AUTHORIZATION,
				header::HeaderValue::from_str(&token.to_string()).expect(
					"Could not parse token value. Make sure it doesn't contain invalid characters.",
				),
			);
		}
		headers
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn get_json_request_value() {
		let json = RequestMaker::new(None)
			.get_json("https://jsonplaceholder.typicode.com/todos/1")
			.unwrap();

		assert_eq!(json["id"], 1);
		assert_eq!(json["completed"], false);
	}

	#[test]
	#[ignore] // This test is kinda slow and annoying, but it should work
	fn given_bad_request_expect_error() {
		let url = "https://jsonplaceholder.typicode.com/dank-memes";
		assert!(
			RequestMaker::new(None).get_json(url).is_err(),
			"Should receive 404"
		);
	}

	#[test]
	fn get_request_that_requires_user_agent() {
		let url = "https://api.github.com/rate_limit"; // rate_limit doesn't incur API hit

		let response = RequestMaker::new(None).get_json(&url).unwrap();
		assert!(
			response.to_string().contains("\"limit\":"),
			"Should receive JSON"
		);
	}
}
