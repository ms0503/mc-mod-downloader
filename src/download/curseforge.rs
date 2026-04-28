use crate::config::Mod;
use crate::constants::CURSEFORGE_API_KEY;
use crate::models::curseforge::GetModFileResponse;
use reqwest::Client;
use std::error::Error;
use std::path::Path;
use tokio::fs::File;
use tokio::io;

#[allow(unreachable_code, unused_variables)]
pub async fn get_file(entry: Mod, dir: &Path) -> Result<(), Box<dyn Error>> {
    unimplemented!("CurseForge Core API requires API key.");
    let Mod::CurseForge {
        file_id,
        name,
        project_id,
        ..
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
    let Mod::CurseForge {
        file_id,
        project_id,
        ..
    } = entry
    else {
        unreachable!()
    };
    let file = Client::new()
        .get(format!(
            "https://api.curseforge.com/v1/mods/{}/files/{}",
            project_id, file_id
        ))
        .header("x-api-key", CURSEFORGE_API_KEY)
        .send()
        .await?
        .json::<GetModFileResponse>()
        .await?;
    Ok(file.data.download_url)
}
