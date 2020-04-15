use reqwest;
use serde::de::DeserializeOwned;

type ErrorBox = Box<dyn std::error::Error>;

#[derive(Default)]
pub struct RequestMaker;

impl RequestMaker {
	pub fn get(url: &str) -> Result<String, ErrorBox> {
		let body = Self::get_response(url)?.text()?;
		Ok(body)
	}

	// todo: wrapper object for json value?
	pub fn get_json(url: &str) -> Result<serde_json::Value, ErrorBox> {
		let json = Self::get_response(url)?.json()?;
		Ok(json)
	}

	pub fn get_json_deserialized<T: DeserializeOwned>(url: &str) -> Result<T, ErrorBox> {
		let object: T = Self::get_response(url)?.json()?;
		Ok(object)
	}

	fn get_response(url: &str) -> Result<reqwest::blocking::Response, ErrorBox> {
		let response = reqwest::blocking::get(url)?;

		let status = response.status();
		if status != 200 {
			return Err(format!("Received status code: {}", status).into());
		}

		Ok(response)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	const TODO_URL: &str = "https://jsonplaceholder.typicode.com/todos/1";
	const ERROR_MESSAGE: &str = "Problem making the request";

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
		let response = RequestMaker::get(TODO_URL).expect(ERROR_MESSAGE);
		assert!(response.contains("\"id\": 1"), "Should receive JSON");
	}

	#[test]
	fn given_bad_request_expect_error() {
		assert!(
			RequestMaker::get("https://jsonplaceholder.typicode.com/dank-memes").is_err(),
			"Should receive 404"
		);
	}

	#[test]
	fn get_json_request_value() {
		let json = RequestMaker::get_json(TODO_URL).expect(ERROR_MESSAGE);

		assert_eq!(json["id"], 1);
		assert_eq!(json["completed"], false);
	}

	#[test]
	fn get_deserialized_json_request_value() {
		let todo: TodoItem = RequestMaker::get_json_deserialized(TODO_URL).expect(ERROR_MESSAGE);

		assert_eq!(todo.id, 1);
		assert_eq!(todo.completed, false);
	}
}
