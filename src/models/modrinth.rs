use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize)]
pub struct GetVersionResponse {
    pub author_id: String,
    pub changelog: Option<String>,
    pub changelog_url: Option<String>,
    pub date_published: String,
    pub dependencies: Vec<Dependency>,
    pub downloads: u64,
    pub featured: bool,
    pub files: Vec<File>,
    pub game_versions: Vec<String>,
    pub id: String,
    pub loaders: Option<Vec<String>>,
    pub name: String,
    pub project_id: String,
    pub requested_status: Option<RequestedStatus>,
    pub status: Status,
    pub version_number: String,
    pub version_type: VersionType
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum VersionType {
    Alpha,
    Beta,
    Release
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Archived,
    Draft,
    Listed,
    Scheduled,
    Unknown,
    Unlisted
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum RequestedStatus {
    Archived,
    Draft,
    Listed,
    Unlisted
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Dependency {}

#[derive(Debug, Deserialize, Serialize)]
pub struct File {
    pub file_type: Option<FileType>,
    pub filename: String,
    pub hashes: Hashes,
    pub primary: bool,
    pub size: u64,
    pub url: String
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum FileType {
    DevJar,
    JavadocJar,
    OptionalResourcePack,
    RequiredResourcePack,
    Signature,
    SourcesJar,
    Unknown
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Hashes {
    pub sha1: String,
    pub sha512: String
}
