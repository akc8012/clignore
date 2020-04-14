use reqwest;
use std::error::Error;

#[derive(Default)]
pub struct RequestMaker {}

impl RequestMaker {
	pub fn new() -> RequestMaker {
		RequestMaker {}
	}

	pub fn make_request(&self, url: &str) -> Result<String, Box<dyn Error>> {
		let response = reqwest::blocking::get(url)?.text()?;
		Ok(response)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn given_get_request_expect_response() {
		let request_maker = RequestMaker::new();
		let response = match request_maker.make_request("https://jsonplaceholder.typicode.com/todos/1") {
			Ok(response) => response,
			Err(error) => panic!("Problem making the request: {}", error)
		};

		assert!(response.contains("\"id\": 1"));
	}
}
