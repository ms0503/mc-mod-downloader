use crate::download::Side;
use clap::Parser;
use clap::Subcommand;
use clap::ValueEnum;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(about, version)]
pub struct Options {
    #[command(subcommand)]
    pub command: Command,
    /// Path to config file
    #[arg(default_value = "mods.json", long, short)]
    pub config: PathBuf
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Download mod(s)
    Download(DownloadOptions),
    // Generate web page of mod list
    GeneratePage(GeneratePageOptions)
}

#[derive(Debug, Parser)]
pub struct DownloadOptions {
    /// Path to mods directory
    #[arg(default_value = "mods", long, short)]
    pub dir: PathBuf,
    /// Whether to include optional mods
    #[arg(long, short = 'o')]
    pub include_optional: bool,
    /// Mod sources to skip
    #[arg(long, value_delimiter = ',')]
    pub skip_source: Vec<Source>,
    /// Side of mod(s) to download
    #[arg(long, short, value_enum)]
    pub side: Side
}

#[derive(Debug, Parser)]
pub struct GeneratePageOptions {
    /// Path to output file
    #[arg(default_value = "mods.html", long, short)]
    pub out: PathBuf
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, ValueEnum)]
pub enum Source {
    CurseForge,
    File,
    Modrinth
}
