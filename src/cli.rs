use crate::download::Sides;
use clap::Parser;
use clap::Subcommand;
use clap::ValueEnum;
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
        /// Whether to include optional mods
        #[arg(long, short = 'o')]
        include_optional: bool,
        /// Mod sources to skip
        #[arg(long, value_delimiter = ',')]
        skip_source: Vec<Source>,
        /// Side of mod(s) to download
        #[arg(long, short, value_enum)]
        side: Sides
    },
    // Generate web page of mod list
    GeneratePage {
        /// Path to output file
        #[arg(default_value = "mods.html", long, short)]
        out: PathBuf
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, ValueEnum)]
pub enum Source {
    CurseForge,
    File,
    Modrinth
}
