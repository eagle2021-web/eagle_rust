use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename = "settings")]
pub struct Settings {
    #[serde(rename = "localRepository")]
    pub local_repository: String,
    // #[serde(rename = "mirrors", skip_serializing_if = "Option::is_none")]
    pub mirrors: Mirrors,
    // #[serde(rename = "profiles", skip_serializing_if = "Option::is_none")]
    pub profiles: Profiles
    // Add more fields as necessary, e.g., servers, proxies, etc.
}

#[derive(Debug, Deserialize)]
pub struct Mirrors {
    pub mirror: Option<Vec<Mirror>>,
}

#[derive(Debug, Deserialize)]
pub struct Mirror {
    pub id: Option<String>,
    pub name: Option<String>,
    pub url: Option<String>,
    #[serde(rename = "mirrorOf")]
    pub mirror_of: Option<String>,
    // Add more elements if necessary
}
#[derive(Debug, Deserialize)]
pub struct Profiles {
    pub profile: Option<Vec<Profile>>
}
#[derive(Debug, Deserialize)]
pub struct Profile {
    // Add fields relevant to the profile
    pub repositories: Option<Repositories>,
    #[serde(rename = "pluginRepositories")]
    pub plugin_repositories: Option<PluginRepositories>,
}

#[derive(Debug, Deserialize)]
pub struct Repositories {
    pub repository: Option<Vec<Repository>>
}
#[derive(Debug, Deserialize)]
pub struct PluginRepositories {
    #[serde(rename = "pluginRepository")]
    pub plugin_repository: Option<Vec<PluginRepository>>
}
#[derive(Debug, Deserialize)]
pub struct Repository {
    pub id: String,
    pub name: String,
    pub url: String,
    // Other properties...
}

#[derive(Debug, Deserialize)]
pub struct PluginRepository {
    pub id: String,
    pub name: String,
    pub url: String,
    // Other properties...
}

