use crate::cli::Commands;
use crate::config::Config;
use crate::config::Mod;
use crate::config::Requirement;
use clap::ValueEnum;
use std::error::Error;
use std::path::Path;
use tokio::fs;
use tokio::task::JoinHandle;

pub(crate) mod curseforge;
pub(crate) mod file;
pub(crate) mod modrinth;

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum Sides {
    /// Download client-side mod(s)
    Client,
    /// Download server-side mod(s)
    Server
}

pub async fn run(command: Commands, config: Config) -> Result<(), Box<dyn Error>> {
    let Commands::Download {
        dir,
        include_optional,
        side
    } = command
    else {
        unreachable!()
    };
    if fs::metadata(&dir).await.is_err() {
        fs::create_dir_all(&dir).await?;
    }
    let mut tasks: Vec<JoinHandle<_>> = vec![];
    for entry in config.mods {
        let dir = dir.clone();
        let name = entry.name().clone();
        match side {
            Sides::Client => tasks.push(tokio::spawn(async move {
                if let Err(err) = get_file(entry.side().client, include_optional, entry, &dir).await
                {
                    eprintln!("[ERROR] {}: Failed to download: {}", name, err);
                }
            })),
            Sides::Server => tasks.push(tokio::spawn(async move {
                if let Err(err) = get_file(entry.side().server, include_optional, entry, &dir).await
                {
                    eprintln!("[ERROR] {}: Failed to download: {}", name, err);
                }
            }))
        }
    }
    for task in tasks {
        task.await?;
    }
    Ok(())
}

async fn get_file(
    requirement: Requirement,
    include_optional: bool,
    entry: Mod,
    dir: &Path
) -> Result<(), Box<dyn Error>> {
    match requirement {
        Requirement::None => Ok(()),
        Requirement::Optional if !include_optional => Ok(()),
        _ => match entry {
            Mod::CurseForge {
                ..
            } => curseforge::get_file(entry, dir).await,
            Mod::File {
                ..
            } => file::get_file(entry, dir).await,
            Mod::Modrinth {
                ..
            } => modrinth::get_file(entry, dir).await
        }
    }
}
