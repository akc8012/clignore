use clap::{App, Arg, SubCommand};
use clignore::Controller;

fn main() {
	let matches = App::new("clignore")
		.version("0.1.0")
		.about("Finds .gitignore template files")
		.author("Andrew Colannino")
		.arg(
			Arg::with_name("token")
				.env("TOKEN")
				.help("Sets the GitHub authorization token"),
		)
		.subcommand(
			SubCommand::with_name("authenticated")
				.about("Checks the GitHub API to see if you have an authorization token present"),
		)
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

	let controller = match matches.value_of("token") {
		Some(token) => Controller::new(Some(token.into())),
		None => Controller::new(None),
	}
	// TODO: Quit unwrapping in here, actually handle the errors (print output?)
	.unwrap();

	if matches.is_present("authenticated") {
		controller.show_is_authenticated().unwrap();
	}

	if matches.is_present("list") {
		controller.list_files().unwrap();
	}

	if let Some(matches) = matches.subcommand_matches("find") {
		controller
			.find_files(matches.value_of("input").unwrap())
			.unwrap();
	}
}
