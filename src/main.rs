use clap::{Arg, Command};
use git2::Repository;
use std::{fs, path::Path, process::exit};

mod constants;
mod utils;

fn main() {
    let matches = Command::new(constants::package::NAME)
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about(constants::package::DESCRIPTION)
        .arg(
            Arg::new(constants::argument::DIRECTORY_LONG)
                .index(1)
                .takes_value(true)
                .help(constants::argument::DIRECTORY_HELP)
                .default_value(constants::default::DIRECTORY)
                .required(false),
        )
        .arg(
            Arg::new(constants::argument::RECURSIVE_LONG)
                .long(constants::argument::RECURSIVE_LONG)
                .short(constants::argument::RECURSIVE_SHORT)
                .help(constants::argument::RECURSIVE_HELP)
                .takes_value(false),
        )
        .get_matches();

    let recurse =
        matches.is_present(constants::argument::RECURSIVE_LONG) || constants::default::RECURSE;
    println!("Recurse: {}", recurse);

    let directory = matches
        .value_of(constants::argument::DIRECTORY_LONG)
        .unwrap();
    match Path::new(directory).exists() {
        true => println!("The directory passed is: {}", directory),
        false => utils::bail(format!("Directory {} does not exist", directory)),
    };

    if !recurse {
        let repository = match Repository::open(directory) {
            Ok(r) => r, //repository = r,
            Err(e) => {
                utils::bail(format!("Failed to open {}: {}", directory, e));
                exit(constants::exit::FAILURE)
            }
        };

        println!(
            "{} state={:?}",
            repository.path().display(),
            repository.state()
        );
    } else {
        println!("Recursively scanning...");
        for entry in fs::read_dir(directory).unwrap() {
            let entry = entry.unwrap();
            if entry.path().is_dir() {
                println!("{:?}", entry);
            }
        }
    }

    exit(constants::exit::SUCCESS)
}
