use auth_token::AuthToken;
use clap::{App, Arg, SubCommand};
use file_finder::FileFinder;
use file_maker::FileMaker;
use github_request_maker::GitHubRequestMaker;
use request_maker::RequestMaker;

mod auth_token;
mod error_box;
mod file_finder;
mod file_maker;
mod github_request_maker;
mod github_url_builder;
mod request_maker;
mod requester;
mod test_request_maker;

fn main() {
	let matches = App::new("ignore_cli")
		.version("0.1.0")
		.about("Finds .gitignore template files")
		.author("Andrew Colannino")
		.subcommand(SubCommand::with_name("list").about("Lists all gitignore template files"))
		.subcommand(
			SubCommand::with_name("find")
				.about("Finds files by name")
				.arg(
					Arg::with_name("input")
						.help("the name to search")
						.index(1)
						.required(true),
				),
		)
		.get_matches();

	let controller = Controller::new();

	if matches.is_present("list") {
		controller.list_files();
	}

	// TODO: Quit unwrapping in here, actually handle the errors (print output?)
	if let Some(matches) = matches.subcommand_matches("find") {
		controller.find_files(matches.value_of("input").unwrap())
	}
}

struct Controller {
	request_maker: GitHubRequestMaker<RequestMaker>,
}

impl Controller {
	pub fn new() -> Controller {
		let request_maker = Self::create_request_maker();
		Controller { request_maker }
	}

	fn create_request_maker() -> GitHubRequestMaker<RequestMaker> {
		let token = AuthToken::new("token.txt");
		let requester = RequestMaker::new(Some(token));
		GitHubRequestMaker::new(requester)
	}

	pub fn list_files(&self) {
		for file_name in &self.get_file_names() {
			println!("{}", file_name);
		}
	}

	pub fn find_files(&self, query: &str) {
		let file_names = self.get_file_names();
		let results = FileFinder::find(&file_names, query);

		if results.len() == 1 {
			self.download_exact_match(results[0]);
		} else if results.len() > 0 {
			for file_name in results {
				println!("{}", file_name);
			}
		} else {
			println!("No matches found for '{}'", query);
		}
	}

	fn get_file_names(&self) -> Vec<String> {
		self.request_maker.get_file_names().unwrap()
	}

	fn download_exact_match(&self, file_name: &str) {
		println!("Found exact match '{}'\nDownloading...", file_name);

		let content = self.request_maker.get_file(file_name).unwrap();
		FileMaker::make_file(".gitignore", &content).unwrap();

		println!("Downloaded '{}'", file_name);
	}
}
