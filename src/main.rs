use clap::App;
mod request_maker;
mod github_request_maker;
mod auth_token;
mod requester;

fn main() {
	App::new("ignore_cli")
		.version("0.1.0")
		.about("Finds .gitignore files")
		.author("Andrew Colannino")
		.get_matches();
}
