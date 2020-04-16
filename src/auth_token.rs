use std::fs::File;
use std::io::Read;

pub struct AuthToken {
	token: String,
}

#[allow(dead_code)] // TODO: REMOVE WHEN CODE IS CALLED IN MAIN!!!!!!!!!
impl AuthToken {
	pub fn new(path: &str) -> AuthToken {
		let token = Self::read_token_from_file(path)
			.unwrap_or_else(|_| panic!("Could not find token file at path: {}", path));

		AuthToken { token }
	}

	fn read_token_from_file(path: &str) -> Result<String, Box<dyn std::error::Error>> {
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
		let token = AuthToken::new("test_token.txt");
		assert_eq!(token.to_string(), "cheese sandwich");
	}

	#[test]
	#[should_panic(expected = "Could not find token file at path: bacon_powder.txt")]
	fn auth_token_error_includes_path() {
		AuthToken::new("bacon_powder.txt");
	}
}
