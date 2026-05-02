use crate::cli::DownloadOptions;
use crate::cli::Source;
use crate::config::Config;
use crate::config::Mod;
use crate::config::Requirement;
use clap::ValueEnum;
use std::error::Error;
use std::path::Path;
use tokio::fs;
use tokio::task::JoinError;
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

pub async fn run(options: DownloadOptions, config: Config) -> Result<(), Box<dyn Error>> {
    let DownloadOptions {
        dir,
        include_optional,
        skip_source,
        side
    } = options;
    if fs::metadata(&dir).await.is_err() {
        fs::create_dir_all(&dir).await?;
    }
    let mut tasks: Vec<JoinHandle<_>> = vec![];
    for entry in config.mods {
        let dir = dir.clone();
        let name = entry.name().clone();
        let skip_source = skip_source.clone();
        match side {
            Sides::Client => tasks.push(tokio::spawn(async move {
                if let Err(err) = get_file(
                    entry.side().client,
                    include_optional,
                    entry,
                    &dir,
                    &skip_source
                )
                .await
                {
                    eprintln!("[ERROR] {}: Failed to download: {}", name, err);
                }
            })),
            Sides::Server => tasks.push(tokio::spawn(async move {
                if let Err(err) = get_file(
                    entry.side().server,
                    include_optional,
                    entry,
                    &dir,
                    &skip_source
                )
                .await
                {
                    eprintln!("[ERROR] {}: Failed to download: {}", name, err);
                }
            }))
        }
    }
    let mut errors: Vec<JoinError> = vec![];
    for task in tasks {
        if let Err(err) = task.await {
            errors.push(err);
        }
    }
    if errors.is_empty() {
        Ok(())
    } else {
        let error = errors
            .iter()
            .map(|err| format!("{}", err))
            .collect::<Vec<_>>()
            .join("\n");
        Err(error.into())
    }
}

async fn get_file(
    requirement: Requirement,
    include_optional: bool,
    entry: Mod,
    dir: &Path,
    skip_source: &[Source]
) -> Result<(), Box<dyn Error>> {
    match requirement {
        Requirement::None => Ok(()),
        Requirement::Optional if !include_optional => Ok(()),
        _ => match entry {
            Mod::CurseForge {
                ..
            } if skip_source
                .iter()
                .find(|s| **s == Source::CurseForge)
                .is_none() =>
                curseforge::get_file(entry, dir).await,
            Mod::File {
                ..
            } if skip_source.iter().find(|s| **s == Source::File).is_none() =>
                file::get_file(entry, dir).await,
            Mod::Modrinth {
                ..
            } if skip_source
                .iter()
                .find(|s| **s == Source::Modrinth)
                .is_none() =>
                modrinth::get_file(entry, dir).await,
            _ => Ok(())
        }
    }
}
