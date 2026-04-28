use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub mods: Vec<Mod>
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase", tag = "source")]
pub enum Mod {
    #[serde(rename_all = "camelCase")]
    CurseForge {
        file_id: u32,
        name: String,
        project_id: u32,
        side: Side
    },
    #[serde(rename_all = "camelCase")]
    File {
        name: String,
        side: Side,
        url: String
    },
    #[serde(rename_all = "camelCase")]
    Modrinth {
        name: String,
        project_id: String,
        side: Side,
        version_id: String
    }
}

impl Mod {
    pub fn name(&self) -> &String {
        match self {
            Self::CurseForge {
                name, ..
            } => name,
            Self::File {
                name, ..
            } => name,
            Self::Modrinth {
                name, ..
            } => name
        }
    }

    pub fn side(&self) -> &Side {
        match self {
            Self::CurseForge {
                side, ..
            } => side,
            Self::File {
                side, ..
            } => side,
            Self::Modrinth {
                side, ..
            } => side
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Side {
    pub client: Requirement,
    pub server: Requirement
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Requirement {
    None,
    Optional,
    Require
}
