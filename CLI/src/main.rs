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
                .help("Sets your name")
                .value_name("NAME"),
        )
        .subcommand(
            Command::new("request")
                .about("Run `cargo run --release` in the ZK-guest workspace")
                .arg(
                    Arg::new("dir")
                        .short('d')
                        .long("dir")
                        .help("Path to the ZK-guest workspace directory")
                        .value_name("PATH")
                        .default_value("ZK-guest"),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("request", sub_m)) => {
            let dir = sub_m
                .get_one::<String>("dir")
                .map(String::as_str)
                .unwrap_or("ZK-guest");

            println!("Running `cargo run --release` in: {}", dir);

            let status = std::process::Command::new("cargo")
                .arg("run")
                .arg("--release")
                .current_dir(dir)
                .status();

            match status {
                Ok(s) => {
                    if let Some(code) = s.code() {
                        std::process::exit(code);
                    } else {
                        eprintln!("Process terminated by signal");
                        std::process::exit(1);
                    }
                }
                Err(e) => {
                    eprintln!("Failed to execute cargo: {}", e);
                    std::process::exit(1);
                }
            }
        }
        _ => {
            if let Some(name) = matches.get_one::<String>("name") {
                println!("Hello, {}!", name);
            } else {
                println!("Hello, world!");
            }
        }
    }
}