use anyhow::{Ok, Result};
use std::{collections::BTreeMap, fs};

use serde::{Deserialize, Serialize};

use crate::vcs::repo::Repo;

#[derive(Serialize, Deserialize, Default)]
pub struct Index {
    pub entries: BTreeMap<String, String>,
}

impl Index {
    /// Load the index from the revision repository
    pub fn load(repo: &Repo) -> Result<Self> {
        let path = repo.path.join("index");

        if !path.exists() {
            return Ok(Self::default());
        }

        let data = fs::read_to_string(path)?;

        Ok(serde_json::from_str(&data)?)
    }

    /// Add an entry to the index, if the entry already exists then we will update it
    pub fn add(&mut self, path: String, hash: String) {
        self.entries.insert(path, hash);
    }

    /// Save the index to the revision repository
    pub fn save(&self, repo: &Repo) -> Result<()> {
        let data = serde_json::to_string_pretty(&self)?;
        fs::write(repo.path.join("index"), data)?;
        Ok(())
    }
}
