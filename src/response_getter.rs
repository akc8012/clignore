use serde::de::DeserializeOwned;

pub type ErrorBox = Box<dyn std::error::Error>;

pub trait ResponseGetter {
	fn get(&self, url: &str) -> Result<String, ErrorBox>;
	fn get_json(&self, url: &str) -> Result<serde_json::Value, ErrorBox>;
	fn get_json_deserialized<T: DeserializeOwned>(&self, url: &str) -> Result<T, ErrorBox>;
}
