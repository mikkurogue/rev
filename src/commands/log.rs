use crate::vcs::repo::Repo;

pub fn show_log() -> anyhow::Result<()> {
    let repo = Repo::discover()?;
    let head_ref = repo.get_head_ref()?;
    let mut current_rev_hash = repo.read_ref(&head_ref)?;

    if current_rev_hash.is_none() {
        println!("No revisions found in the repository.");
        return Ok(());
    }

    while let Some(rev_hash) = current_rev_hash.clone() {
        match repo.get_revision(&rev_hash) {
            Ok(revision) => {
                println!("\x1b[33mrevision {}\x1b[0m", rev_hash);
                println!("Author: {}", revision.author);
                println!("Date:   {}", revision.date);
                println!("\n    {}\n", revision.message);
                println!(
                    "Parent: {}\n",
                    revision.parent.clone().unwrap_or("ORPHANED".to_string())
                );

                if let Some(parent_hash) = revision.parent {
                    current_rev_hash = Some(parent_hash);
                } else {
                    break;
                }
            }
            Err(_) => {
                // Parent revision not found, stop here
                break;
            }
        }
    }

    Ok(())
}
