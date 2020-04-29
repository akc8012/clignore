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

impl Controller {
	// TODO: THIS FLAG IS TEMPORARY FOR TESTS!!!!!!!!!!!!!!!
	pub fn new(create_token: bool) -> Result<Controller, ErrorBox> {
		let request_maker = Self::create_request_maker(create_token)?;
		Ok(Controller { request_maker })
	}

	fn create_request_maker(
		create_token: bool,
	) -> Result<GitHubRequestMaker<RequestMaker>, ErrorBox> {
		if create_token {
			let token = AuthToken::new("token.txt")?;
			let requester = RequestMaker::new(Some(token));

			return Ok(GitHubRequestMaker::new(requester));
		}

		let requester = RequestMaker::new(None);
		let request_maker = GitHubRequestMaker::new(requester);

		// TODO: Handle unauthenticated error when we see limit has hit 0
		if let Some(limit) = request_maker.too_close_to_limit()? {
			println!(
				"Warning: The GitHub API will only allow you to make {} more requests for the hour. Consider providing an authentication token.",
				limit
			)
		}

		Ok(request_maker)
	}

	// pub fn set_token(token_string: &str) {
	// 	let token = AuthToken::new("token.txt");
	// 	let requester = RequestMaker::new(Some(token));

	// 	Ok(GitHubRequestMaker::new(requester))
	// }

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
			self.download_exact_match(&results[0])?;
		} else if !results.is_empty() {
			self.handle_multiple_matches(query, results)?;
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

	fn handle_multiple_matches(&self, query: &str, results: Vec<String>) -> Result<(), ErrorBox> {
		println!("Several matches found for '{}':\n", query);

		let choice_presenter = ChoicePresenter::new(results);
		println!("{}\n", choice_presenter.present_choices());

		loop {
			match self.get_choice(&choice_presenter) {
				Ok(choice) => match choice {
					Some(choice) => return self.download_exact_match(&choice),
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
