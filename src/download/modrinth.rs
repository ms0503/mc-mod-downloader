use crate::config::Mod;
use crate::models::modrinth::GetVersionResponse;
use lazy_regex::regex;
use reqwest::Url;
use std::error::Error;
use std::path::Path;
use tokio::fs::File;
use tokio::io;

const MODRINTH_API_GET_VERSION_BASE: &str = "/v2/version";
const MODRINTH_API_HOST: &str = "api.modrinth.com";
const MODRINTH_DOWNLOAD_BASE: &str = "/data";
const MODRINTH_DOWNLOAD_HOST: &str = "cdn.modrinth.com";

pub async fn get_file(entry: Mod, dir: &Path) -> Result<(), Box<dyn Error>> {
    let Mod::Modrinth {
        name,
        project_id,
        version_id,
        ..
    } = &entry
    else {
        unreachable!()
    };
    let _ = validate_id(project_id)?;
    let _ = validate_id(version_id)?;
    let out_filename = {
        let mut path = dir.to_path_buf();
        path.push(name);
        path
    };
    let url = get_download_url(&entry).await?;
    let validated_download_url = validate_download_url(&url)?;
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
    let version_id = validate_id(version_id)?;
    let api_url = validate_api_url(version_id)?;
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

fn validate_id(id: &str) -> Result<&str, Box<dyn Error>> {
    let pattern = regex!(r#"^[\w!@$()`.+,"\-']{3,64}$"#);
    if pattern.is_match(id) {
        Ok(id)
    } else {
        Err("Invalid Modrinth ID".into())
    }
}

fn validate_api_url(version_id: &str) -> Result<Url, Box<dyn Error>> {
    let url = Url::parse(&format!(
        "https://{}{}/{}",
        MODRINTH_API_HOST, MODRINTH_API_GET_VERSION_BASE, version_id
    ))?;
    let is_valid = url.scheme() == "https"
        && url.host_str() == Some(MODRINTH_API_HOST)
        && url.username().is_empty()
        && url.password().is_none()
        && (url.port().is_none() || url.port() == Some(443))
        && url
            .path()
            .starts_with(&format!("{}/", MODRINTH_API_GET_VERSION_BASE));
    if is_valid {
        Ok(url)
    } else {
        Err("Invalid Modrinth API URL".into())
    }
}

fn validate_download_url(raw: &str) -> Result<Url, Box<dyn Error>> {
    let url = Url::parse(raw)?;
    let is_valid = url.scheme() == "https"
        && url.host_str() == Some(MODRINTH_DOWNLOAD_HOST)
        && url.username().is_empty()
        && url.password().is_none()
        && (url.port().is_none() || url.port() == Some(443))
        && url
            .path()
            .starts_with(&format!("{}/", MODRINTH_DOWNLOAD_BASE));
    if is_valid {
        Ok(url)
    } else {
        Err("Invalid Modrinth download URL".into())
    }
}
