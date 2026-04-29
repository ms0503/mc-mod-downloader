use crate::config::Mod;
use crate::models::modrinth::GetVersionResponse;
use reqwest::Url;
use std::error::Error;
use std::path::Path;
use tokio::fs::File;
use tokio::io;

const MODRINTH_API_HOST: &str = "api.modrinth.com";
const MODRINTH_DOWNLOAD_HOST: &str = "cdn.modrinth.com";

pub async fn get_file(entry: Mod, dir: &Path) -> Result<(), Box<dyn Error>> {
    let Mod::Modrinth {
        name, ..
    } = &entry
    else {
        unreachable!()
    };
    let out_filename = {
        let mut path = dir.to_path_buf();
        path.push(name);
        path
    };
    let url = get_download_url(&entry).await?;
    let validated_download_url = validate_modrinth_download_url(&url)?;
    let data = reqwest::get(validated_download_url).await?.bytes().await?;
    let mut out_file = File::create(out_filename).await?;
    io::copy(&mut data.as_ref(), &mut out_file).await?;
    println!("[INFO] {}: Downloaded", name);
    Ok(())
}

pub async fn get_download_url(entry: &Mod) -> Result<String, Box<dyn Error>> {
    let Mod::Modrinth {
        name,
        version_id,
        ..
    } = entry
    else {
        unreachable!()
    };
    let api_url = validate_modrinth_api_url(version_id)?;
    let version = reqwest::get(api_url)
        .await?
        .json::<GetVersionResponse>()
        .await?;
    let file = version
        .files
        .iter()
        .find(|file| file.filename == *name)
        .unwrap();
    Ok(file.url.clone())
}

fn validate_modrinth_api_url(version_id: &str) -> Result<Url, Box<dyn Error>> {
    let url = Url::parse(&format!(
        "https://api.modrinth.com/v2/version/{}",
        version_id
    ))?;
    if url.scheme() == "https" && url.host_str() == Some(MODRINTH_API_HOST) {
        Ok(url)
    } else {
        Err("Invalid Modrinth API URL".into())
    }
}

fn validate_modrinth_download_url(raw: &str) -> Result<Url, Box<dyn Error>> {
    let url = Url::parse(raw)?;
    if url.scheme() == "https" && url.host_str() == Some(MODRINTH_DOWNLOAD_HOST) {
        Ok(url)
    } else {
        Err("Invalid Modrinth download URL".into())
    }
}
