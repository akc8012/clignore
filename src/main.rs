use clap::App;

use ignore_cli::RequestMaker;

fn main() {
	App::new("ignore-cli")
		.version("0.1.0")
		.about("Finds gitignore files")
		.author("Andrew Colannino")
		.get_matches();

	RequestMaker::make_request();
}
