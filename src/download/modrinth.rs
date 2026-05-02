use crate::config::Mod;
use crate::models::modrinth::GetVersionResponse;
use crate::url::validate_path;
use lazy_regex::Lazy;
use lazy_regex::Regex;
use lazy_regex::lazy_regex;
use reqwest::Url;
use std::error::Error;
use std::path::Path;
use tokio::fs::File;
use tokio::io;

const MODRINTH_API_GET_VERSION_BASE: &str = "/v2/version";
const MODRINTH_API_HOST: &str = "api.modrinth.com";
const MODRINTH_DOWNLOAD_HOST: &str = "cdn.modrinth.com";

static MODRINTH_APT_GET_VERSION_PATH_PATTERN: Lazy<Regex> =
    lazy_regex!(r#"\A/v2/version/([\w.\-]|%(21|22|24|27|28|29|2B|2C|40|60)){3,64}\z"#);
static MODRINTH_DOWNLOAD_PATH_PATTERN: Lazy<Regex> = lazy_regex!(
    r#"\A/data/([\w.\-]|%(21|22|24|27|28|29|2B|2C|40|60)){3,64}/versions/([\w.\-]|%(21|22|24|27|28|29|2B|2C|40|60)){3,64}/([\w.\-~]|%[0-9A-F]{2})+\.(jar|zip)\z"#
);
static MODRINTH_ID_PATTERN: Lazy<Regex> = lazy_regex!(r#"\A[\w!@$()`.+,"\-']{3,64}\z"#);

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
    let url = validate_canonicalize_download_url(&url)?;
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
    let version_id = validate_id(version_id)?;
    let version_id = urlencoding::encode(version_id);
    let api_url = validate_api_url(&version_id)?;
    let version = reqwest::get(api_url)
        .await?
        .json::<GetVersionResponse>()
        .await?;
    let file = version
        .files
        .iter()
        .find(|file| file.filename == *name)
        .unwrap();
    let url = validate_canonicalize_download_url(&file.url)?;
    Ok(url.to_string())
}

fn validate_id(id: &str) -> Result<&str, Box<dyn Error>> {
    if MODRINTH_ID_PATTERN.is_match(id) {
        Ok(id)
    } else {
        Err("Invalid Modrinth ID".into())
    }
}

fn validate_api_url(version_id: &str) -> Result<Url, Box<dyn Error>> {
    let url = {
        let mut url = Url::parse(&format!("https://{}", MODRINTH_API_HOST))?;
        url.set_path(&format!("{}/{}", MODRINTH_API_GET_VERSION_BASE, version_id));
        url
    };
    let is_valid = url.scheme() == "https"
        && url.host_str() == Some(MODRINTH_API_HOST)
        && url.username().is_empty()
        && url.password().is_none()
        && (url.port().is_none() || url.port() == Some(443))
        && validate_path(url.path()).is_ok()
        && MODRINTH_APT_GET_VERSION_PATH_PATTERN.is_match(url.path());
    if is_valid {
        Ok(url)
    } else {
        Err("Invalid Modrinth API URL".into())
    }
}

fn validate_canonicalize_download_url(raw: &str) -> Result<Url, Box<dyn Error>> {
    let url = Url::parse(raw)?;
    let is_valid = url.scheme() == "https"
        && url.host_str() == Some(MODRINTH_DOWNLOAD_HOST)
        && url.username().is_empty()
        && url.password().is_none()
        && (url.port().is_none() || url.port() == Some(443))
        && validate_path(url.path()).is_ok()
        && MODRINTH_DOWNLOAD_PATH_PATTERN.is_match(url.path());
    if is_valid {
        let mut canonical_url = Url::parse(&format!("https://{}", MODRINTH_DOWNLOAD_HOST))?;
        canonical_url.set_path(url.path());
        Ok(canonical_url)
    } else {
        Err("Invalid Modrinth download URL".into())
    }
}
