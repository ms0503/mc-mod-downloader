use crate::download::Sides;
use clap::Parser;
use clap::Subcommand;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(about, version)]
pub struct Options {
    #[command(subcommand)]
    pub command: Commands,
    /// Path to config file
    #[arg(default_value = "mods.json", long, short)]
    pub config: PathBuf
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Download mod(s)
    Download {
        /// Path to mods directory
        #[arg(default_value = "mods", long, short)]
        dir: PathBuf,
        #[arg(long, short = 'o')]
        include_optional: bool,
        /// Side of mod(s) to download
        #[arg(long, short, value_enum)]
        side: Sides
    }
}
