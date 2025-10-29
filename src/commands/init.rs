use std::fs::{create_dir_all, write};
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
    // this should probably be something better
    // but for now it's just fine
    // eventually a global config that then scopes to .rev repos by using
    // rev init --global
    // for a global config
    // and
    // rev init --local author.name "name" author.email
    // "email" for local configs would be smarter
    // would mean we need to give the user an "interactive" installation of the tool
    // so they can set up a global config, this should then live in $HOME/.revconfig or similar
    write(
        rev_dir.join("config"),
        "author_name = Local\nauthor_email = user@local",
    )?;

    write(rev_dir.join("HEAD"), "ref: refs/heads/main\n")?;

    println!("Initialized empty revision repository in .rev/");

    Ok(())
}
