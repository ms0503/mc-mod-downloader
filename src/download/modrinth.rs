use crate::config::Mod;
use crate::models::modrinth::GetVersionResponse;
use std::error::Error;
use std::path::Path;
use tokio::fs::File;
use tokio::io;

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
    let data = reqwest::get(url).await?.bytes().await?;
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
    let version = reqwest::get(format!(
        "https://api.modrinth.com/v2/version/{}",
        version_id
    ))
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
