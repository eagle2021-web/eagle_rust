use serde::Deserialize;
#[derive(Debug, Deserialize)]
pub struct Dependency {
    #[serde(rename = "groupId")]
    pub group_id: Option<String>,
    #[serde(rename = "artifactId")]
    pub artifact_id: Option<String>,
    pub version: Option<String>,
    pub scope: Option<String>
}