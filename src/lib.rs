use reqwest;
use std::error::Error;

#[derive(Default)]
pub struct RequestMaker {}

impl RequestMaker {
	pub fn new() -> RequestMaker {
		RequestMaker {}
	}

	pub fn make_request(&self, url: &str) -> Result<String, Box<dyn Error>> {
		let response = reqwest::blocking::get(url)?;

		let status = response.status();
		if status != 200 {
			return Err(format!("eceived status code: {}", status).into());
		}
		let body = response.text()?;
		Ok(body)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn given_request_expect_response() {
		let request_maker = RequestMaker::new();
		let response =
			match request_maker.make_request("https://jsonplaceholder.typicode.com/todos/1") {
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
				.make_request("https://jsonplaceholder.typicode.com/dank-memes")
				.is_err(),
			"Should receive 404"
		);
	}
}
