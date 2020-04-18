use auth_token::AuthToken;
use clap::{App, SubCommand};
use github_request_maker::GitHubRequestMaker;
use request_maker::RequestMaker;

mod auth_token;
mod github_request_maker;
mod request_maker;
mod requester;
mod test_request_maker;

fn main() {
	let matches = App::new("ignore_cli")
		.version("0.1.0")
		.about("Finds .gitignore template files")
		.author("Andrew Colannino")
		.subcommand(SubCommand::with_name("list").about("Lists all gitignore template files"))
		.get_matches();

	if matches.is_present("list") {
		list_files();
	}
}

fn list_files() {
	let token = AuthToken::new("token.txt"); // maybe return Result, Err when file not found (make it optional, warn when not found)
	let requester = RequestMaker::new(Some(token));
	let request_maker = GitHubRequestMaker::new(requester);

	println!(
		"{:?}",
		request_maker
			.get_file_names(&request_maker.get_tree_id().unwrap())
			.unwrap()
	);
}
