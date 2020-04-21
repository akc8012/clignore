use auth_token::AuthToken;
use choice_presenter::ChoicePresenter;
use error_box::ErrorBox;
use file_finder::FileFinder;
use file_maker::FileMaker;
use github_request_maker::GitHubRequestMaker;
use request_maker::RequestMaker;

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
		for file_name in &self.get_file_names()? {
			println!("{}", file_name);
		}

		Ok(())
	}

	pub fn find_files(&self, query: &str) -> Result<(), ErrorBox> {
		let file_names = self.get_file_names()?;
		let results = FileFinder::find(&file_names, query);

		if results.len() == 1 {
			self.download_exact_match(results[0])?;
		} else if !results.is_empty() {
			// TODO: Fix string Vec cloning
			let results: Vec<String> = results.iter().map(|s| s.to_string()).collect();
			self.handle_multiple_matches(query, &results);
		} else {
			println!("No matches found for '{}'", query);
		}

		Ok(())
	}

	fn get_file_names(&self) -> Result<Vec<String>, ErrorBox> {
		self.request_maker.get_file_names()
	}

	fn download_exact_match(&self, file_name: &str) -> Result<(), ErrorBox> {
		println!("Found exact match '{}'\nDownloading...", file_name);

		let content = self.request_maker.get_file(file_name)?;
		FileMaker::make_file(".gitignore", &content)?;

		println!("Downloaded '{}'", file_name);
		Ok(())
	}

	fn handle_multiple_matches(&self, query: &str, results: &[String]) {
		println!("Several matches found for '{}':\n", query);

		let choice_presenter = ChoicePresenter::new(results);
		println!("{}\n", choice_presenter.present_choices());

		println!(
			"Which do you want to use (0 to abort)? [0-{}]:",
			choice_presenter.len()
		);
	}
}
