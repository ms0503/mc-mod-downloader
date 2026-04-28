use serde::Deserialize;
use serde::Serialize;
use serde_repr::Deserialize_repr;
use serde_repr::Serialize_repr;

#[derive(Debug, Deserialize, Serialize)]
pub struct GetModFileResponse {
    pub data: File
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct File {
    pub alternate_file_id: Option<u32>,
    pub dependencies: Vec<FileDependency>,
    pub display_name: String,
    pub download_count: u64,
    pub download_url: String,
    pub early_access_end_date: Option<String>,
    pub expose_as_alternative: Option<bool>,
    pub file_date: String,
    pub file_fingerprint: u64,
    pub file_length: u64,
    pub file_name: String,
    pub file_size_on_disk: Option<u64>,
    pub file_status: FileStatus,
    pub game_id: u32,
    pub game_versions: Vec<String>,
    pub hashes: Vec<FileHash>,
    pub id: u32,
    pub is_available: bool,
    pub is_early_access_content: Option<bool>,
    pub is_server_pack: Option<bool>,
    pub mod_id: u32,
    pub modules: Vec<FileModule>,
    pub parent_project_file_id: Option<u32>,
    pub release_type: FileReleaseType,
    pub server_pack_file_id: Option<u32>,
    pub sortable_game_versions: Vec<SortableGameVersion>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FileModule {
    pub fingerprint: u64,
    pub name: String
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FileDependency {
    pub mod_id: u32,
    pub relation_type: FileRelationType
}

#[derive(Debug, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum FileRelationType {
    EmbeddedLibrary = 1,
    OptionalDependency = 2,
    RequiredDependency = 3,
    Tool = 4,
    Incompatible = 5,
    Include = 6
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SortableGameVersion {
    pub game_version: String,
    pub game_version_name: String,
    pub game_version_padded: String,
    pub game_version_release_date: String,
    pub game_version_type_id: Option<u32>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FileHash {
    pub algo: HashAlgo,
    pub value: String
}

#[derive(Debug, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum HashAlgo {
    Sha1 = 1,
    Md5 = 2
}

#[derive(Debug, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum FileReleaseType {
    Release = 1,
    Beta = 2,
    Alpha = 3
}

#[derive(Debug, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum FileStatus {
    Processing = 1,
    ChangesRequired = 2,
    UnderReview = 3,
    Approved = 4,
    Rejected = 5,
    MalwareDetected = 6,
    Deleted = 7,
    Archived = 8,
    Testing = 9,
    Released = 10,
    ReadyForReview = 11,
    Deprecated = 12,
    Baking = 13,
    AwaitingPublishing = 14,
    FailedPublishing = 15,
    Cooking = 16,
    Cooked = 17,
    UnderManualReview = 18,
    ScanningForMalware = 19,
    ProcessingFile = 20,
    PendingRelease = 21,
    ReadyForCooking = 22,
    PostProcessing = 23
}
