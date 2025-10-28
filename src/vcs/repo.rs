use anyhow::{Ok, Result, anyhow};
use std::fs;
use std::fs::write;
use std::path::{Path, PathBuf};

pub struct Repo {
    pub path: PathBuf,
}

impl Repo {
    /// Discover an existing revision repository in the current directory
    pub fn discover() -> Result<Self> {
        let path = Path::new(".rev");

        if !path.exists() {
            return Err(anyhow!(
                "not a rev repository (tip: run `rev init` to create a new revision repository)"
            ));
        }

        Ok(Self {
            path: path.to_path_buf(),
        })
    }

    /// Get the path to the objects directory for the repository
    pub fn get_objects_dir(&self) -> PathBuf {
        self.path.join("objects")
    }

    /// Get the current HEAD reference
    pub fn get_head_ref(&self) -> Result<String> {
        let head = fs::read_to_string(self.path.join("HEAD"))?;

        Ok(head.trim_start_matches("ref:").trim().to_string())
    }

    /// Read the ref of a given name
    pub fn read_ref(&self, name: &str) -> Result<Option<String>> {
        let ref_path = self.path.join(name);

        if ref_path.exists() {
            let contents = fs::read_to_string(ref_path)?;
            let trimmed = contents.trim();

            if trimmed.is_empty() {
                return Ok(None);
            }

            Ok(Some(trimmed.to_string()))
        } else {
            Ok(None)
        }
    }

    /// Update the ref of a given name to point to the new hash
    pub fn update_ref(&self, name: &str, hash: &str) -> Result<()> {
        let ref_path = self.path.join(name);
        write(ref_path, hash)?;
        Ok(())
    }
}
