use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

const FREEDICT_API_URL: &str = "https://freedict.org/freedict-database.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FreeDictRelease {
    #[serde(rename = "URL")]
    pub url: String,
    pub checksum: String,
    pub date: String,
    #[serde(deserialize_with = "deserialize_size")]
    pub size: u64,
}

fn deserialize_size<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::Error;
    let s: String = Deserialize::deserialize(deserializer)?;
    s.parse::<u64>().map_err(D::Error::custom)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FreeDictEntry {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub releases: Vec<FreeDictRelease>,
    #[serde(default)]
    pub headwords: String,
    #[serde(default)]
    pub status: String,
}

impl FreeDictEntry {
    pub fn is_valid(&self) -> bool {
        !self.name.is_empty() && !self.releases.is_empty()
    }

    pub fn get_dictd_release(&self) -> Option<&FreeDictRelease> {
        self.releases
            .iter()
            .find(|r| r.url.contains(".dictd.tar.xz"))
    }
}

pub fn fetch_available_dictionaries() -> Result<Vec<FreeDictEntry>> {
    let response =
        reqwest::blocking::get(FREEDICT_API_URL).context("Failed to fetch dictionary database")?;

    let mut entries: Vec<FreeDictEntry> = response
        .json()
        .context("Failed to parse dictionary database")?;

    entries.retain(|e| e.is_valid());

    Ok(entries)
}
