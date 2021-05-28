use clap::{crate_authors, crate_version, App, Arg};
use git2::Repository;
use std::{path::Path, process::exit};

mod constants;
mod utils;

fn main() {
    let matches = App::new(constants::package::NAME)
        .version(crate_version!())
        .author(crate_authors!())
        .about(constants::package::DESCRIPTION)
        .arg(
            Arg::with_name(constants::argument::DIRECTORY_LONG)
                .index(1)
                .takes_value(true)
                .help(constants::argument::DIRECTORY_HELP)
                .default_value(constants::default::DIRECTORY)
                .required(false),
        )
        .arg(
            Arg::with_name(constants::argument::RECURSIVE_LONG)
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

    exit(constants::exit::SUCCESS)
}
