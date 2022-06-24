use std::{path::Path, process::exit};

use clap::{Arg, Command};
use walkdir::WalkDir;

mod constants;

fn main() {
    let args = Command::new(constants::package::NAME)
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about(constants::package::DESCRIPTION)
        .arg(
            Arg::new(constants::argument::DIRECTORY_LONG)
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
        args.is_present(constants::argument::RECURSIVE_LONG) || constants::default::RECURSE;

    let dir = args.value_of(constants::argument::DIRECTORY_LONG).unwrap();

    if !recurse {
        // most common use case would be to scan the current directory or a specific one
        analyze_directory(Path::new(dir));
    } else {
        println!("Recursively scanning [{}]...", dir);

        for entry in WalkDir::new(dir).min_depth(1).max_depth(1) {
            match entry {
                Ok(entry) => analyze_directory(entry.path()),
                Err(error) => {
                    eprintln!("Could not walk directory [{}]: {} ", dir, error);
                    exit(constants::exit::FAILURE);
                }
            }
        }
    }

    exit(constants::exit::SUCCESS)
}

/// Scans the given `dir` for git repository information.
fn analyze_directory(dir: &Path) {
    match dir.exists() && dir.is_dir() {
        true => {
            println!("Scanning [{}]...", dir.to_str().unwrap());

            match Repository::open(dir) {
                Ok(repo) => {
                    println!("{} state={:?}", repo.path().display(), repo.state());
                    println!("bare:{}", repo.is_bare());

                    match repo.config() {
                        Ok(config) => {
                            for entry in &config.entries(None).unwrap() {
                                let entry = entry.unwrap();

                                println!(
                                    "config {} => {}",
                                    entry.name().unwrap(),
                                    entry.value().unwrap()
                                );
                            }
                        }
                        Err(_) => todo!(),
                    }

                    match repo.branches(None) {
                        Ok(branches) => {
                            for branch in branches.enumerate() {
                                println!("branch #{}: {:?}", branch.0, branch.1.unwrap().0.name());
                            }
                        }
                        Err(e) => eprintln!("{}", e),
                    }
                }
                Err(e) => {
                    eprintln!(
                        "Failed to open repository [{}]: {}",
                        dir.to_str().unwrap(),
                        e
                    );
                }
            };
        }
        false => eprintln!(
            "Directory [{}] does not exist or is not a directory",
            dir.to_str().unwrap()
        ),
    };
}
