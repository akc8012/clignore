use auth_token::AuthToken;
use clap::{App, Arg, SubCommand};
use file_finder::FileFinder;
use github_request_maker::GitHubRequestMaker;
use request_maker::RequestMaker;

mod auth_token;
mod error_box;
mod file_finder;
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

	if matches.is_present("list") {
		list_files();
	}

	// TODO: Quit unwrapping in here, actually handle the errors (print output?)
	if let Some(matches) = matches.subcommand_matches("find") {
		find_files(matches.value_of("input").unwrap());
	}
}

fn list_files() {
	let file_names = get_file_names();
	for file_name in file_names {
		println!("{}", file_name);
	}
}

fn find_files(query: &str) {
	let file_names = get_file_names();
	let results = FileFinder::find(&file_names, query);

	if results.len() > 0 {
		for result in results {
			println!("{}", result);
		}
	} else {
		println!("No matches found for '{}'", query);
	}
}

fn get_file_names() -> Vec<String> {
	let token = AuthToken::new("token.txt");
	let requester = RequestMaker::new(Some(token));
	let request_maker = GitHubRequestMaker::new(requester);

	request_maker.get_file_names().unwrap()
}
