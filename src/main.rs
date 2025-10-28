use clap::{Parser, Subcommand};

mod commands;
mod vcs;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new revision repository
    Init,
    /// Add a file or directory to be tracked
    Track { path: String },
    /// Create a new revision with a message - similar to git commit
    Revise { message: String },
    // /// Show the revision log for the current branch
    // Log,
    // /// Create a new revision branch from the current revision branch
    // Branch {
    //     name: Option<String>,
    // },
    // /// Merge a target revision into the current revision branch
    // Merge {
    //     branch: String,
    // },
    // /// swap to a different revision branch
    // Swap {
    //     branch: String,
    // },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => commands::init::init_repo()?,
        Commands::Track { path } => commands::track::track_file(&path)?,
        Commands::Revise { message } => commands::revise::revise(&message)?,
        _ => println!("Command not implemented yet"),
        // Commands::Add { path } => commands::add::add_path(&path)?,
        // Commands::Revise { message } => commands::revise::create_revision(&message)?,
        // Commands::Log => commands::log::show_log()?,
        // Commands::Branch { name } => commands::branch::create_branch(name)?,
        // Commands::Merge { branch } => commands::merge::merge_branch(&branch)?,
        // Commands::Swap { branch } => commands::swap::swap_branch(&branch)?,
    }

    Ok(())
}
