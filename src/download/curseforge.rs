use crate::config::Mod;
use crate::constants::CURSEFORGE_API_KEY;
use crate::models::curseforge::GetModFileResponse;
use reqwest::Client;
use reqwest::Url;
use std::error::Error;
use std::path::Path;
use tokio::fs::File;
use tokio::io;

const CURSEFORGE_API_GET_MOD_BASE: &str = "/v1/mods";
const CURSEFORGE_API_HOST: &str = "api.curseforge.com";
const CURSEFORGE_DOWNLOAD_BASE: &str = "/files";
const CURSEFORGE_DOWNLOAD_HOST: &str = "mediafilez.forgecdn.net";

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
    let url = validate_curseforge_download_url(&url)?;
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
    let api_url = validate_curseforge_api_url(*project_id, *file_id)?;
    let file = Client::new()
        .get(api_url)
        .header("x-api-key", CURSEFORGE_API_KEY)
        .send()
        .await?
        .json::<GetModFileResponse>()
        .await?;
    Ok(file.data.download_url)
}

fn validate_curseforge_api_url(project_id: u32, file_id: u32) -> Result<Url, Box<dyn Error>> {
    let url = Url::parse(&format!(
        "https://{}{}/{}/files/{}/download-url",
        CURSEFORGE_API_HOST, CURSEFORGE_API_GET_MOD_BASE, project_id, file_id
    ))?;
    let is_valid = url.scheme() == "https"
        && url.host_str() == Some(CURSEFORGE_API_HOST)
        && url.username().is_empty()
        && url.password().is_none()
        && (url.port().is_none() || url.port() == Some(443))
        && url
            .path()
            .starts_with(&format!("{}/", CURSEFORGE_API_GET_MOD_BASE));
    if is_valid {
        Ok(url)
    } else {
        Err("Invalid CurseForge API URL".into())
    }
}

fn validate_curseforge_download_url(raw: &str) -> Result<Url, Box<dyn Error>> {
    let url = Url::parse(raw)?;
    let is_valid = url.scheme() == "https"
        && url.host_str() == Some(CURSEFORGE_DOWNLOAD_HOST)
        && url.username().is_empty()
        && url.password().is_none()
        && (url.port().is_none() || url.port() == Some(443))
        && url
            .path()
            .starts_with(&format!("{}/", CURSEFORGE_DOWNLOAD_BASE));
    if is_valid {
        Ok(url)
    } else {
        Err("Invalid CurseForge download URL".into())
    }
}
