use clap::App;

use ignore_cli::RequestMaker;

fn main() {
	App::new("ignore_cli")
		.version("0.1.0")
		.about("Finds .gitignore files")
		.author("Andrew Colannino")
		.get_matches();

	let request_maker = RequestMaker::new();
	request_maker.make_request();
}
