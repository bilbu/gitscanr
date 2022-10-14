use std::{env, path::Path, process::exit};

use clap::{Arg, Command};
use git2::Repository;
use log::{debug, error, info};
use walkdir::WalkDir;

extern crate pretty_env_logger;

mod constants;
use constants::{arg, default, package};

fn main() {
    let args = Command::new(package::NAME)
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about(package::DESCRIPTION)
        .arg(
            Arg::new(arg::DIRECTORY_LONG)
                .takes_value(true)
                .help(arg::DIRECTORY_HELP)
                .default_value(default::DIRECTORY)
                .required(false),
        )
        .arg(
            Arg::new(arg::RECURSIVE_LONG)
                .long(arg::RECURSIVE_LONG)
                .short(arg::RECURSIVE_SHORT)
                .help(arg::RECURSIVE_HELP)
                .takes_value(false),
        )
        .arg(
            Arg::new(arg::DEBUG_LONG)
                .long(arg::DEBUG_LONG)
                .short(arg::DEBUG_SHORT)
                .help(arg::DEBUG_HELP)
                .takes_value(false),
        )
        .get_matches();

    let log_level = match args.is_present(arg::DEBUG_LONG) {
        true => default::LOG_LVL_DEBUG,
        false => default::LOG_LVL,
    };

    env::set_var("RUST_LOG", log_level);
    pretty_env_logger::init();

    let recurse = args.is_present(arg::RECURSIVE_LONG) || default::RECURSE;

    let dir = args.value_of(arg::DIRECTORY_LONG).unwrap();

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
                            let mut entries = config.entries(None).unwrap();

                            while let Some(entry) = entries.next() {
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
