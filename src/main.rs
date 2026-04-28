use clap::Parser;
use mc_mod_downloader::cli::Commands;
use mc_mod_downloader::cli::Options;
use mc_mod_downloader::config::Config;
use mc_mod_downloader::download;
use mc_mod_downloader::generate_page;
use std::error::Error;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let opts = Options::parse();
    let mut config = File::open(&opts.config).await?;
    let config = {
        let mut buf = String::new();
        config.read_to_string(&mut buf).await?;
        buf
    };
    let config: Config = match opts
        .config
        .extension()
        .expect("Cannot guess config file type")
        .to_str()
        .unwrap()
    {
        "json" => serde_json::from_str(&config)?,
        "toml" => toml::from_str(&config)?,
        "yaml" | "yml" => serde_yaml::from_str(&config)?,
        t => panic!("Unsupported file type: {}", t)
    };
    match opts.command {
        Commands::Download {
            ..
        } => download::run(opts.command, config).await,
        Commands::GeneratePage {
            ..
        } => generate_page::run(opts.command, config).await
    }
}
