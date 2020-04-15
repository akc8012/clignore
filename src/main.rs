use clap::App;
mod request_maker;

fn main() {
	App::new("ignore_cli")
		.version("0.1.0")
		.about("Finds .gitignore files")
		.author("Andrew Colannino")
		.get_matches();
}
