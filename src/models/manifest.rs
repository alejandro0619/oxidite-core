use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Manifest {
    pub latest: Latest,
    pub versions: Vec<Version>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Latest {
    pub release: String,
    pub snapshot: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Version {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub url: String,
    pub time: String,
    pub release_time: String,
}

impl Manifest {
    pub fn find_version(&self, id: &str) -> Option<&Version> {
        self.versions.iter().find(|v| v.id == id)
    }
}

impl Version {
    pub fn is_release(&self) -> bool {
        self.type_field == "release"
    }

    pub fn is_snapshot(&self) -> bool {
        self.type_field == "snapshot"
    }

    pub fn is_old_alpha(&self) -> bool {
        self.type_field == "old_alpha"
    }

    pub fn get_hash(&self) -> Option<String> {
        self.url
            .split("/packages/")
            .nth(1)? // Takes the part after "/packages/"
            .split('/') // Splits the remaining string by '/'
            .next() // The first element is the hash
            .map(|s| s.to_string())
    }
}
