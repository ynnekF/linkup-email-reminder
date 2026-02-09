use crate::core::errors::CliError;
use std::path::PathBuf;

pub struct Context {
    pub verbose: bool,
    pub resources_dir: PathBuf,
}

impl Context {
    pub fn new() -> Result<Self, CliError> {
        Ok(Context {
            verbose: false,
            resources_dir: PathBuf::from("resources"),
        })
    }

    pub fn set_verbose(&mut self, verbose: bool) {
        self.verbose = verbose;
    }
}
