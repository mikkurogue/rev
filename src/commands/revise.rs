use crate::vcs::{index::Index, object, repo};
use chrono::Utc;

pub fn revise(message: &str) -> anyhow::Result<()> {
    let repo = repo::Repo::discover()?;
    let index = Index::load(&repo)?;

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

    let revision_data = format!(
        "
    type=revision\n
    tree={}\n\
    parent={}\n\
    author=rev user <me@local>\n
    date={}\n
    message={}\n",
        tree_hash,
        parent.unwrap_or_default(),
        Utc::now(),
        message
    );

    let revision_hash = object::store_revision(&repo, &revision_data)?;

    repo.update_ref(&head_ref, &revision_hash)?;

    println!("rev: revised ->  {} ({})", message, revision_hash);
    Ok(())
}
