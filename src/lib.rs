use reqwest;
use std::error::Error;

#[derive(Default)]
pub struct RequestMaker {}

impl RequestMaker {
	pub fn new() -> RequestMaker {
		RequestMaker {}
	}

	pub fn get(&self, url: &str) -> Result<String, Box<dyn Error>> {
		let body = self.get_response(url)?.text()?;
		Ok(body)
	}

	pub fn get_json(&self, url: &str) -> Result<serde_json::Value, Box<dyn Error>> {
		let json = self.get_response(url)?.json()?;
		Ok(json)
	}

	fn get_response(&self, url: &str) -> Result<reqwest::blocking::Response, Box<dyn Error>> {
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
	use serde_json::json;

	#[test]
	fn given_request_expect_response() {
		let request_maker = RequestMaker::new();
		let response = match request_maker.get("https://jsonplaceholder.typicode.com/todos/1") {
			Ok(response) => response,
			Err(error) => panic!("Problem making the request: {}", error),
		};

		assert!(response.contains("\"id\": 1"), "Should receive JSON");
	}

	#[test]
	fn given_bad_request_expect_error() {
		let request_maker = RequestMaker::new();
		assert!(
			request_maker
				.get("https://jsonplaceholder.typicode.com/dank-memes")
				.is_err(),
			"Should receive 404"
		);
	}

	#[test]
	fn get_deserialized_json() {
		let object = json!({
			"A": ["a", "á", "à"],
			"B": ["b", "b́"],
			"C": ["c", "ć", "ć̣", "ḉ"],
		});

		assert_eq!(object["B"][0], json!("b"));
		assert_eq!(object["B"][0], "b");
	}

	#[test]
	fn get_json_request_value() {
		let request_maker = RequestMaker::new();
		let json = match request_maker.get_json("https://jsonplaceholder.typicode.com/todos/1") {
			Ok(json) => json,
			Err(error) => panic!("Problem making the request: {}", error),
		};

		assert_eq!(json["id"], 1);
		assert_eq!(json["completed"], false);
	}
}
