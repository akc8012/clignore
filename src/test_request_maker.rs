use crate::requester::{ErrorBox, Requester};
use serde::de::DeserializeOwned;

pub struct TestRequestMaker;

impl Requester for TestRequestMaker {
	fn get(&self, _url: &str) -> Result<String, ErrorBox> {
		panic!("I have no definition and I must scream");
	}

	// TODO: wrapper object for json value?
	fn get_json(&self, _url: &str) -> Result<serde_json::Value, ErrorBox> {
		Ok(serde_json::json!(
			// "dank" hashed with sha1
			[{"commit": { "tree": { "sha": "9431e108b67d1efa9df54e6351da1951bcd9be32" } }}]
		))
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
