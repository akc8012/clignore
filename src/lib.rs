use reqwest;
use std::error::Error;

#[derive(Default)]
pub struct RequestMaker {}

impl RequestMaker {
	pub fn new() -> RequestMaker {
		RequestMaker {}
	}

	pub fn make_request(&self) -> Result<String, Box<dyn Error>> {
		let body = reqwest::blocking::get("https://www.rust-lang.org")?.text()?;
		Ok(body)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn cheese() {
		let request_maker = RequestMaker::new();
		match request_maker.make_request() {
			Ok(body) => println!("body = {:?}", body),
			Err(error) => panic!("Problem making the request: {}", error)
		}
	}
}
