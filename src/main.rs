use std::{env, path::Path, process::exit};

use clap::{Arg, Command};
use git2::Repository;
use log::{debug, error, info};
use walkdir::WalkDir;

extern crate pretty_env_logger;

mod constants;

use constants::{
    argument::{
        DEBUG_HELP, DEBUG_LONG, DEBUG_SHORT, DIRECTORY_HELP, DIRECTORY_LONG, RECURSIVE_HELP,
        RECURSIVE_LONG, RECURSIVE_SHORT,
    },
    default::{DEBUG, DIRECTORY, RECURSE},
    package::{DESCRIPTION, NAME},
};

fn main() {
    let args = Command::new(NAME)
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about(DESCRIPTION)
        .arg(
            Arg::new(DIRECTORY_LONG)
                .takes_value(true)
                .help(DIRECTORY_HELP)
                .default_value(DIRECTORY)
                .required(false),
        )
        .arg(
            Arg::new(RECURSIVE_LONG)
                .long(RECURSIVE_LONG)
                .short(RECURSIVE_SHORT)
                .help(RECURSIVE_HELP)
                .takes_value(false),
        )
        .arg(
            Arg::new(DEBUG_LONG)
                .long(DEBUG_LONG)
                .short(DEBUG_SHORT)
                .help(DEBUG_HELP)
                .takes_value(false),
        )
        .get_matches();

    if args.is_present(DEBUG_LONG) {
        env::set_var("RUST_LOG", DEBUG);
    }
    pretty_env_logger::init();

    let recurse = args.is_present(RECURSIVE_LONG) || RECURSE;

    let dir = args.value_of(DIRECTORY_LONG).unwrap();

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
                            while let Some(entry) = config.entries(None).unwrap().next() {
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
