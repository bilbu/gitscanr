use std::{path::Path, process::exit};

use clap::{Arg, Command};
use git2::Repository;
use log::{debug, error, info};
use walkdir::WalkDir;

extern crate pretty_env_logger;

mod constants;

fn main() {
    pretty_env_logger::init();

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
        info!("Recursively scanning [{}]...", dir);

        for entry in WalkDir::new(dir).min_depth(1).max_depth(1) {
            match entry {
                Ok(entry) => analyze_directory(entry.path()),
                Err(error) => {
                    error!("Could not walk directory [{}]: {} ", dir, error);
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
            info!("Scanning [{}]...", dir.to_str().unwrap());

            match Repository::open(dir) {
                Ok(repo) => {
                    debug!("{} state={:?}", repo.path().display(), repo.state());
                    debug!("bare:{}", repo.is_bare());

                    match repo.config() {
                        Ok(config) => {
                            for entry in &config.entries(None).unwrap() {
                                let entry = entry.unwrap();

                                debug!(
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
                            for (i, val) in branches.enumerate() {
                                let (branch, _) = val.unwrap();
                                debug!("branch #{}: {:?}", i, branch.name());
                            }
                        }
                        Err(e) => eprintln!("{}", e),
                    }
                }
                Err(e) => {
                    error!(
                        "Failed to open repository [{}]: {}",
                        dir.to_str().unwrap(),
                        e
                    );
                }
            };
        }
        false => error!(
            "Directory [{}] does not exist or is not a directory",
            dir.to_str().unwrap()
        ),
    };
}
