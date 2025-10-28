use std::fs;

use crate::vcs::{index::Index, object, repo};

pub fn track_file(path: &str) -> anyhow::Result<()> {
    let repo = repo::Repo::discover()?;
    let contents = fs::read(path)?;
    let hash = object::store_blob(&repo, &contents)?;

    let mut index = Index::load(&repo)?;

    index.add(path.to_string(), hash.clone());

    index.save(&repo)?;

    println!("rev: now tracking {} with hash {}", path, &hash[..8]);

    Ok(())
}
