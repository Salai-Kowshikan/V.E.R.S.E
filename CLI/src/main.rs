use clap::{Arg, Command};

fn main() {
	let matches = Command::new("verse")
		.version("1.0")
		.author("Your Name <your.email@example.com>")
		.about("A simple CLI tool built with clap")
		.arg(
			Arg::new("name")
				.short('n')
				.long("name")
				.help("Sets your name"),
		)
		.get_matches();

	if let Some(name) = matches.get_one::<String>("name") {
		println!("Hello, {}!", name);
	} else {
		println!("Hello, world!");
	}
}
