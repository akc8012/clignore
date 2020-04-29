use crate::error_box::ErrorBox;

pub trait Requester {
	fn get_json(&self, url: &str) -> Result<serde_json::Value, ErrorBox>;
}
