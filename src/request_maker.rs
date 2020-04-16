use reqwest::{blocking, header};
use serde::de::DeserializeOwned;
use std::fs::File;
use std::io::Read;

// TODO: Make pub, to use in GitHubRequestMaker
type ErrorBox = Box<dyn std::error::Error>;

#[derive(Default)]
pub struct RequestMaker;

#[allow(dead_code)] // TODO: REMOVE WHEN CODE IS CALLED IN MAIN!!!!!!!!!
impl RequestMaker {
	pub fn new() -> RequestMaker {
		RequestMaker {}
	}

	pub fn get(&self, url: &str) -> Result<String, ErrorBox> {
		let body = self.get_response(url)?.text()?;
		Ok(body)
	}

	// TODO: wrapper object for json value?
	pub fn get_json(&self, url: &str) -> Result<serde_json::Value, ErrorBox> {
		self.get_json_deserialized(url)
	}

	pub fn get_json_deserialized<T: DeserializeOwned>(&self, url: &str) -> Result<T, ErrorBox> {
		let object: T = self.get_response(url)?.json()?;
		Ok(object)
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

		let mut token = String::new();
		File::open("token.txt")
			.unwrap()
			.read_to_string(&mut token)
			.unwrap();

		let mut headers = header::HeaderMap::new();
		headers.insert(
			header::AUTHORIZATION,
			header::HeaderValue::from_str(&token).unwrap(),
		);

		blocking::Client::builder()
			.user_agent(user_agent)
			.default_headers(headers)
			.build()
			.expect("Can't create client")
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
		let response = RequestMaker::new().get(TODO_URL).expect(ERROR_MESSAGE);
		assert!(response.contains("\"id\": 1"), "Should receive JSON");
	}

	#[test]
	fn given_bad_request_expect_error() {
		let url = "https://jsonplaceholder.typicode.com/dank-memes";
		assert!(RequestMaker::new().get(url).is_err(), "Should receive 404");
	}

	#[test]
	fn get_json_request_value() {
		let json = RequestMaker::new().get_json(TODO_URL).expect(ERROR_MESSAGE);

		assert_eq!(json["id"], 1);
		assert_eq!(json["completed"], false);
	}

	#[test]
	fn get_deserialized_json_request_value() {
		let todo: TodoItem = RequestMaker::new()
			.get_json_deserialized(TODO_URL)
			.expect(ERROR_MESSAGE);

		assert_eq!(todo.id, 1);
		assert_eq!(todo.completed, false);
	}

	#[test]
	fn get_request_that_requires_user_agent() {
		let url = "https://api.github.com/rate_limit"; // rate_limit doesn't incur API hit

		let response = RequestMaker::new().get(&url).expect(ERROR_MESSAGE);
		assert!(response.contains("\"limit\":"), "Should receive JSON");
	}

	#[test]
	fn get_request_with_custom_header() {
		let url = "https://api.github.com/rate_limit";

		let json = RequestMaker::new().get_json(&url).expect(ERROR_MESSAGE);
		assert_eq!(json["resources"]["core"]["limit"], 5000); // rate_limit at 5000 when using auth header
	}
}
