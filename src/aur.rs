use reqwest::Client;
use serde::{Deserialize, Serialize};

const AUR_BASE_URL: &str = "https://aur.archlinux.org";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AurPackage {
    pub name: String,
    pub version: String,
    pub description: String,
    pub maintainer: Option<String>,
    pub url: Option<String>,
    pub license: Vec<String>,
    #[serde(rename = "NumVotes")]
    pub num_votes: i32,
    pub popularity: f64,
    #[serde(rename = "FirstSubmitted")]
    pub firstsubmitted: String,
    #[serde(rename = "LastModified")]
    pub lastmodified: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct SearchResponse {
    #[serde(rename = "resultcount")]
    result_count: i32,
    results: Vec<SearchResult>,
}

#[derive(Debug, Serialize, Deserialize)]
struct SearchResult {
    name: String,
    version: String,
    description: String,
    maintainer: Option<String>,
    #[serde(rename = "NumVotes")]
    num_votes: i32,
    popularity: f64,
}

impl From<SearchResult> for AurPackage {
    fn from(r: SearchResult) -> Self {
        Self {
            name: r.name,
            version: r.version,
            description: r.description,
            maintainer: r.maintainer,
            url: None,
            license: vec![],
            num_votes: r.num_votes,
            popularity: r.popularity,
            firstsubmitted: String::new(),
            lastmodified: String::new(),
        }
    }
}

pub struct AurClient {
    client: Client,
}

impl AurClient {
    pub fn new() -> Self {
        let client = Client::builder()
            .user_agent("bearch-aur/0.1.0")
            .build()
            .expect("Failed to create HTTP client");
        Self { client }
    }

    pub async fn search(&self, query: &str) -> Result<Vec<AurPackage>, Box<dyn std::error::Error>> {
        let url = format!("{}/rpc/v5/search/{}", AUR_BASE_URL, query);
        let response = self.client.get(&url).send().await?;
        let search: SearchResponse = response.json().await?;
        Ok(search.results.into_iter().map(AurPackage::from).collect())
    }

    pub async fn info(&self, package: &str) -> Result<AurPackage, Box<dyn std::error::Error>> {
        let url = format!("{}/rpc/v5/info/{}", AUR_BASE_URL, package);
        let response = self.client.get(&url).send().await?;
        let info: Vec<AurPackage> = response.json().await?;
        info.into_iter()
            .next()
            .ok_or_else(|| "Package not found".into())
    }
}

impl Default for AurClient {
    fn default() -> Self {
        Self::new()
    }
}