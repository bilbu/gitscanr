pub mod package {
    pub const NAME: &str = env!("CARGO_PKG_NAME");
    pub const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
}

pub mod default {
    pub const DIRECTORY: &str = "./";
    pub const RECURSE: bool = false;
}

pub mod argument {
    pub const DIRECTORY_LONG: &str = "directory"; // TODO: reuse/concatenate somehow below
    pub const DIRECTORY_HELP: &str = "The directory to scan";
    pub const RECURSIVE_LONG: &str = "recursive";
    pub const RECURSIVE_SHORT: char = 'r';
    pub const RECURSIVE_HELP: &str = "When [directory] is the root of multiple git reopsitories";
}

// TODO: work around exit codes' portability cross platform
pub mod exit {
    pub const SUCCESS: i32 = 0;
    pub const FAILURE: i32 = 1;
}
