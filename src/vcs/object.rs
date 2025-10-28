use anyhow::Result;
use sha1::{Digest, Sha1};
use std::fs;
use std::io::Write;

use crate::vcs::repo::Repo;

fn hash_bytes(data: &[u8]) -> String {
    let mut hasher = Sha1::new();
    hasher.update(data);
    let result = hasher.finalize();
    hex::encode(result)
}

fn store_object(repo: &Repo, content: &[u8]) -> Result<String> {
    let hash = hash_bytes(content);
    let object_path = repo.get_objects_dir().join(&hash);

    if !object_path.exists() {
        let mut file = fs::File::create(object_path)?;
        file.write_all(content)?;
    }

    Ok(hash)
}

pub fn store_blob(repo: &Repo, data: &[u8]) -> Result<String> {
    store_object(repo, data)
}

pub fn store_tree(repo: &Repo, data: &str) -> Result<String> {
    store_object(repo, data.as_bytes())
}

pub fn store_revision(repo: &Repo, data: &str) -> Result<String> {
    store_object(repo, data.as_bytes())
}
