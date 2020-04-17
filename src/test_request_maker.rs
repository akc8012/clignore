use crate::requester::{ErrorBox, Requester};
use serde::de::DeserializeOwned;

pub struct TestRequestMaker;

impl Requester for TestRequestMaker {
	fn get(&self, _url: &str) -> Result<String, ErrorBox> {
		panic!("I have no definition and I must scream");
	}

	// TODO: wrapper object for json value?
	fn get_json(&self, url: &str) -> Result<serde_json::Value, ErrorBox> {
		if url.contains("/commits") {
			Ok(serde_json::json!(
				// "dank" hashed with sha1
				[{"commit": { "tree": { "sha": "9431e108b67d1efa9df54e6351da1951bcd9be32" } }}]
			))
		} else {
			Ok(serde_json::json!(
				{ "tree": [
					{ "path": "yeet.gitignore" },
					{ "path": "yoink.gitignore" },
					{ "path": "quite.gitignore" }
				] }
			))
		}
	}

	fn get_json_deserialized<T: DeserializeOwned>(&self, _url: &str) -> Result<T, ErrorBox> {
		panic!("I have no definition and I must scream");
	}
}

#[allow(dead_code)] // TODO: REMOVE WHEN CODE IS CALLED IN MAIN!!!!!!!!!
impl TestRequestMaker {
	pub fn new() -> TestRequestMaker {
		TestRequestMaker {}
	}
}
