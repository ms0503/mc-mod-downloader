use crate::config::Mod;
use std::error::Error;
use std::path::Path;
use tokio::fs::File;
use tokio::io;

pub async fn get_file(entry: Mod, dir: &Path) -> Result<(), Box<dyn Error>> {
    let Mod::File {
        name, ..
    } = &entry
    else {
        unreachable!()
    };
    println!(
        "[WARN] {}: File URL downloader may pose a security risk",
        name
    );
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
    if let Mod::File {
        url, ..
    } = entry
    {
        Ok(url.clone())
    } else {
        Ok("javascript:void(0)".to_string())
    }
}
