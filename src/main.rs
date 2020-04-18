use clap::App;

mod auth_token;
mod github_request_maker;
mod github_url_builder;
mod request_maker;
mod requester;
mod test_request_maker;

fn main() {
	App::new("ignore_cli")
		.version("0.1.0")
		.about("Finds .gitignore files")
		.author("Andrew Colannino")
		.get_matches();
}
