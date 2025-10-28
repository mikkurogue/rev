use std::fs::{self, create_dir_all, write};
use std::io::Result;
use std::path::Path;

/// Initialize a new `revision repository` in the current directory.
pub fn init_repo() -> Result<()> {
    let rev_dir = Path::new(".rev");

    if rev_dir.exists() {
        println!("Revision repository already exists in this location");
        return Ok(());
    }

    create_dir_all(rev_dir.join("objects"))?;
    create_dir_all(rev_dir.join("refs/heads"))?;

    write(rev_dir.join("HEAD"), "ref: refs/heads/main\n")?;

    println!("Initialized empty revision repository in .rev/");

    Ok(())
}
