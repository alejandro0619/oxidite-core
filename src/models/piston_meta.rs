use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PistonMeta {
    pub arguments: Arguments,
    pub asset_index: AssetIndex,
    pub assets: String,
    pub compliance_level: Option<i64>,
    pub downloads: Downloads,
    pub id: String,
    pub java_version: JavaVersion,
    pub libraries: Vec<Library>,
    pub logging: Option<Logging>,
    pub main_class: String,
    pub minimum_launcher_version: i64,
    pub release_time: String,
    pub time: String,
    #[serde(rename = "type")]
    pub type_field: String,
}
impl PistonMeta {
    /// Returns a user-friendly message about Java compatibility based on the user's current Java version.
    pub fn get_java_status(&self, current_version: i64) -> String {
        if current_version >= self.java_version.major_version {
            format!("Compatible: Java {} detected (minimum {})", current_version, self.java_version.major_version)
        } else {
            format!("Incompatible: Java {} is required, but you have {}", self.java_version.major_version, current_version)
        }
    }
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Arguments {
    pub game: Vec<Value>,
    pub jvm: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetIndex {
    pub id: String,
    pub sha1: String,
    pub size: i64,
    pub total_size: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Downloads {
    pub client: Client,
    #[serde(rename = "client_mappings")]
    pub client_mappings: Option<ClientMappings>,
    pub server: Server,
    #[serde(rename = "server_mappings")]
    pub server_mappings: Option<ServerMappings>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Client {
    pub sha1: String,
    pub size: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientMappings {
    pub sha1: String,
    pub size: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Server {
    pub sha1: String,
    pub size: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerMappings {
    pub sha1: String,
    pub size: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JavaVersion {
    pub component: String,
    pub major_version: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Library {
    pub downloads: LibraryDownloads,
    pub name: String,
    #[serde(default)]
    pub rules: Vec<Rule4>,
}

impl Library {
    pub fn is_compatible(&self) -> bool {
        if self.rules.is_empty() {
            return true;
        }

        let mut allow = false;
        for rule in &self.rules {
            // Look if the rule has an 'os' condition and if it matches the current OS
            let os_matches = if let Some(os_rule) = &rule.os {
                // Minecraft usa "windows", "linux", "osx"
                os_rule.name == "windows" 
            } else {
                // If not, the library applies to all OSes
                true
            };

            if os_matches {
                allow = rule.action == "allow";
            }
        }
        allow
    }
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LibraryDownloads {
    pub artifact: Artifact,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Artifact {
    pub path: String,
    pub sha1: String,
    pub size: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rule4 {
    pub action: String,
    pub os: Option<Os4>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Os4 {
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Logging {
    pub client: Client2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Client2 {
    pub argument: String,
    pub file: File,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct File {
    pub id: String,
    pub sha1: String,
    pub size: i64,
    pub url: String,
}
