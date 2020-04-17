use crate::auth_token::AuthToken;
use crate::requester::{ErrorBox, Requester};

use reqwest::{blocking, header};
use serde::de::DeserializeOwned;

#[derive(Default)]
pub struct RequestMaker {
	token: Option<AuthToken>,
}

impl Requester for RequestMaker {
	fn get(&self, url: &str) -> Result<String, ErrorBox> {
		let body = self.get_response(url)?.text()?;
		Ok(body)
	}

	// TODO: wrapper object for json value?
	fn get_json(&self, url: &str) -> Result<serde_json::Value, ErrorBox> {
		self.get_json_deserialized(url)
	}

	fn get_json_deserialized<T: DeserializeOwned>(&self, url: &str) -> Result<T, ErrorBox> {
		let object: T = self.get_response(url)?.json()?;
		Ok(object)
	}
}

#[allow(dead_code)] // TODO: REMOVE WHEN CODE IS CALLED IN MAIN!!!!!!!!!
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
			.expect("Can't create client")
	}

	fn get_headers(&self) -> header::HeaderMap {
		let mut headers = header::HeaderMap::new();

		if let Some(token) = &self.token {
			headers.insert(
				header::AUTHORIZATION,
				header::HeaderValue::from_str(&token.to_string()).expect("wtf"),
			);
		}
		headers
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	const TODO_URL: &str = "https://jsonplaceholder.typicode.com/todos/1";
	const ERROR_MESSAGE: &str = "Problem making the request"; // TODO: Make pub?

	#[derive(serde::Deserialize, Debug)]
	#[allow(non_snake_case)]
	struct TodoItem {
		userId: i32,
		id: i32,
		title: String,
		completed: bool,
	}

	#[test]
	fn given_request_expect_response() {
		let response = RequestMaker::new(None).get(TODO_URL).expect(ERROR_MESSAGE);
		assert!(response.contains("\"id\": 1"), "Should receive JSON");
	}

	#[test]
	fn given_bad_request_expect_error() {
		let url = "https://jsonplaceholder.typicode.com/dank-memes";
		assert!(
			RequestMaker::new(None).get(url).is_err(),
			"Should receive 404"
		);
	}

	#[test]
	fn get_json_request_value() {
		let json = RequestMaker::new(None)
			.get_json(TODO_URL)
			.expect(ERROR_MESSAGE);

		assert_eq!(json["id"], 1);
		assert_eq!(json["completed"], false);
	}

	#[test]
	fn get_deserialized_json_request_value() {
		let todo: TodoItem = RequestMaker::new(None)
			.get_json_deserialized(TODO_URL)
			.expect(ERROR_MESSAGE);

		assert_eq!(todo.id, 1);
		assert_eq!(todo.completed, false);
	}

	#[test]
	fn get_request_that_requires_user_agent() {
		let url = "https://api.github.com/rate_limit"; // rate_limit doesn't incur API hit

		let response = RequestMaker::new(None).get(&url).expect(ERROR_MESSAGE);
		assert!(response.contains("\"limit\":"), "Should receive JSON");
	}
}
