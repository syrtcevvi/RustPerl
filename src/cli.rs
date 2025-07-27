use std::path::PathBuf;

pub use clap::Parser;

#[derive(Debug, Parser)]
#[command(version)]
pub struct CliArgs {
    pub path_to_script: Option<PathBuf>,
}
