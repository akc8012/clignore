use clap::App;

fn main() {
	App::new("ignore-cli")
		.version("0.1.0")
		.about("Finds gitignore files")
		.author("Andrew Colannino")
		.get_matches();

	println!("yahoo");
}
