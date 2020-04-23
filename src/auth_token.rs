use crate::error_box::ErrorBox;
use std::fs::File;
use std::io::Read;

pub struct AuthToken {
	token: String,
}

#[allow(dead_code)] // TODO: yeet me into the sun
impl AuthToken {
	pub fn new(path: &str) -> Result<AuthToken, ErrorBox> {
		match Self::read_token_from_file(path) {
			Ok(token) => Ok(AuthToken { token }),
			Err(_) => Err(format!("Could not find token file at path: {}", path).into()),
		}
	}

	fn read_token_from_file(path: &str) -> Result<String, ErrorBox> {
		let mut token = String::new();
		File::open(path)?.read_to_string(&mut token)?;

		Ok(token)
	}
}

impl std::string::ToString for AuthToken {
	fn to_string(&self) -> String {
		self.token.clone()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn auth_token_opens_file() {
		let token = AuthToken::new("test_token.txt").unwrap();
		assert_eq!(token.to_string(), "cheese sandwich");
	}

	#[test]
	#[should_panic(expected = "Could not find token file at path: bacon_powder.txt")]
	fn auth_token_error_includes_path() {
		AuthToken::new("bacon_powder.txt").unwrap();
	}
}
