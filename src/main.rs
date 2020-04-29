use clap::{App, Arg, SubCommand};
use ignore_cli::Controller;

fn main() {
	let token_arg = Arg::with_name("token")
		.short("t")
		.long("token")
		.help("Sets the GitHub authorization token")
		.takes_value(true);

	let matches = App::new("ignore_cli")
		.version("0.1.0")
		.about("Finds .gitignore template files")
		.author("Andrew Colannino")
		.subcommand(
			SubCommand::with_name("list")
				.about("Lists all gitignore template files")
				.arg(&token_arg),
		)
		.subcommand(
			SubCommand::with_name("find")
				.about("Finds files by name")
				.arg(
					Arg::with_name("input")
						.help("the name to search")
						.index(1)
						.required(true),
				)
				.arg(&token_arg),
		)
		.get_matches();

	// TODO: Quit unwrapping in here, actually handle the errors (print output?)
	let controller = Controller::new(true).unwrap();

	if matches.is_present("list") {
		controller.list_files().unwrap();
	}

	if let Some(matches) = matches.subcommand_matches("find") {
		controller
			.find_files(matches.value_of("input").unwrap())
			.unwrap();
	}
}
