use crate::config::Mod;
use crate::constants::CURSEFORGE_API_KEY;
use crate::models::curseforge::GetModFileResponse;
use crate::url::validate_path;
use lazy_regex::Lazy;
use lazy_regex::Regex;
use lazy_regex::lazy_regex;
use reqwest::Client;
use reqwest::Url;
use std::error::Error;
use std::path::Path;
use tokio::fs::File;
use tokio::io;

const CURSEFORGE_API_GET_MOD_BASE: &str = "/v1/mods";
const CURSEFORGE_API_HOST: &str = "api.curseforge.com";
const CURSEFORGE_DOWNLOAD_HOST: &str = "mediafilez.forgecdn.net";

static CURSEFORGE_API_GET_MOD_PATH_PATTERN: Lazy<Regex> =
    lazy_regex!(r#"\A/v1/mods/[0-9]+/files/[0-9]+/download-url\z"#);
static CURSEFORGE_DOWNLOAD_PATH_PATTERN: Lazy<Regex> =
    lazy_regex!(r#"\A/files/[0-9]+/[0-9]+/([\w.\-~]|%[0-9A-F]{2})+\.(jar|zip)\z"#);

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
    let url = validate_canonicalize_download_url(&url)?;
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
    let api_url = validate_api_url(*project_id, *file_id)?;
    let file = Client::new()
        .get(api_url)
        .header("x-api-key", CURSEFORGE_API_KEY)
        .send()
        .await?
        .json::<GetModFileResponse>()
        .await?;
    let url = validate_canonicalize_download_url(&file.data.download_url)?;
    Ok(url.to_string())
}

fn validate_api_url(project_id: u32, file_id: u32) -> Result<Url, Box<dyn Error>> {
    let url = {
        let mut url = Url::parse(&format!("https://{}", CURSEFORGE_API_HOST))?;
        url.set_path(&format!(
            "{}/{}/files/{}/download-url",
            CURSEFORGE_API_GET_MOD_BASE, project_id, file_id
        ));
        url
    };
    let is_valid = url.scheme() == "https"
        && url.host_str() == Some(CURSEFORGE_API_HOST)
        && url.username().is_empty()
        && url.password().is_none()
        && (url.port().is_none() || url.port() == Some(443))
        && validate_path(url.path()).is_ok()
        && CURSEFORGE_API_GET_MOD_PATH_PATTERN.is_match(url.path());
    if is_valid {
        Ok(url)
    } else {
        Err("Invalid CurseForge API URL".into())
    }
}

fn validate_canonicalize_download_url(raw: &str) -> Result<Url, Box<dyn Error>> {
    let url = Url::parse(raw)?;
    let is_valid = url.scheme() == "https"
        && url.host_str() == Some(CURSEFORGE_DOWNLOAD_HOST)
        && url.username().is_empty()
        && url.password().is_none()
        && (url.port().is_none() || url.port() == Some(443))
        && validate_path(url.path()).is_ok()
        && CURSEFORGE_DOWNLOAD_PATH_PATTERN.is_match(url.path());
    if is_valid {
        let mut canonical_url = Url::parse(&format!("https://{}", CURSEFORGE_DOWNLOAD_HOST))?;
        canonical_url.set_path(url.path());
        canonical_url.set_query(url.query());
        Ok(canonical_url)
    } else {
        Err("Invalid CurseForge download URL".into())
    }
}
