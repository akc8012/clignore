use auth_token::AuthToken;
use choice_presenter::{ChoicePresenter, ChoiceResult};
use error_box::ErrorBox;
use file_finder::FileFinder;
use file_maker::FileMaker;
use github_request_maker::GitHubRequestMaker;
use request_maker::RequestMaker;
use std::io;

mod auth_token;
mod choice_presenter;
mod error_box;
mod file_finder;
mod file_maker;
mod github_request_maker;
mod github_url_builder;
mod request_maker;
mod requester;
mod test_request_maker;

pub struct Controller {
	request_maker: GitHubRequestMaker<RequestMaker>,
}

// TODO: Integration tests
impl Controller {
	pub fn new() -> Result<Controller, ErrorBox> {
		let request_maker = Self::create_request_maker()?;
		Ok(Controller { request_maker })
	}

	fn create_request_maker() -> Result<GitHubRequestMaker<RequestMaker>, ErrorBox> {
		let token = AuthToken::new("token.txt")?;
		let requester = RequestMaker::new(Some(token));

		Ok(GitHubRequestMaker::new(requester))
	}

	pub fn list_files(&self) -> Result<(), ErrorBox> {
		for file_name in self.request_maker.get_file_names()? {
			println!("{}", file_name);
		}

		Ok(())
	}

	pub fn find_files(&self, query: &str) -> Result<(), ErrorBox> {
		let file_names = self.request_maker.get_file_names()?;
		let results = FileFinder::find(&file_names, query);

		if results.len() == 1 {
			self.download_exact_match(results[0])?;
		} else if !results.is_empty() {
			// TODO: Fix Vec<String> cloning
			let results: Vec<String> = results.iter().map(|s| (*s).to_string()).collect();
			self.handle_multiple_matches(query, &results)?;
		} else {
			println!("No matches found for '{}'", query);
		}

		Ok(())
	}

	fn download_exact_match(&self, file_name: &str) -> Result<(), ErrorBox> {
		println!("Found exact match '{}'\nDownloading...", file_name);

		let content = self.request_maker.get_file(file_name)?;
		FileMaker::make_file(".gitignore", &content)?;

		println!("Downloaded '{}'", file_name);
		Ok(())
	}

	fn handle_multiple_matches(&self, query: &str, results: &[String]) -> Result<(), ErrorBox> {
		println!("Several matches found for '{}':\n", query);

		let choice_presenter = ChoicePresenter::new(results);
		println!("{}\n", choice_presenter.present_choices());

		loop {
			match self.get_choice(&choice_presenter) {
				Ok(choice) => match choice {
					Some(choice) => return self.download_exact_match(choice),
					None => return Ok(()),
				},
				Err(_) => {
					println!("Invalid input, please try again.");
					continue;
				}
			}
		}
	}

	fn get_choice<'c>(&self, choice_presenter: &'c ChoicePresenter) -> ChoiceResult<'c> {
		println!(
			"Which do you want to use (0 to cancel)? [0-{}]:",
			choice_presenter.len()
		);

		let input = self.get_choice_input();
		choice_presenter.select_choice(&input)
	}

	fn get_choice_input(&self) -> String {
		let mut choice = String::new();
		io::stdin().read_line(&mut choice).unwrap();
		choice
	}
}
