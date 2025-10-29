use crate::{
    config::Config,
    vcs::{index::Index, object, repo},
};
use chrono::Utc;

pub struct Revision {
    pub hash: String,
    pub message: String,
    pub author: String,
    pub date: String,
    pub parent: Option<String>,
    pub tree_hash: String,
}

pub fn revise(message: &str) -> anyhow::Result<()> {
    let repo = repo::Repo::discover()?;
    let index = Index::load(&repo)?;
    let config = Config::load();

    if index.entries.is_empty() {
        println!("rev: nothing to revise");
        return Ok(());
    }

    let tree_data = index
        .entries
        .iter()
        .map(|(path, hash)| format!("{} {}", hash, path))
        .collect::<Vec<_>>()
        .join("\n");

    let tree_hash = object::store_tree(&repo, &tree_data)?;
    let head_ref = repo.get_head_ref()?;
    let parent = repo.read_ref(&head_ref)?;

    let revision = Revision {
        hash: String::new(),
        message: message.to_string(),
        author: format!("{} <{}>", config.author.name, config.author.email),
        date: Utc::now().to_string(),
        parent,
        tree_hash,
    };

    let revision_hash = object::store_revision(&repo, revision)?;

    repo.update_ref(&head_ref, &revision_hash)?;

    println!("rev: revised ->  {} ({})", message, revision_hash);
    Ok(())
}
