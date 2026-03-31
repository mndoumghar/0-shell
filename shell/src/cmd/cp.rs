use crate::error::ShellError;
use std::collections::HashSet;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct Cp {
    pub targets: Vec<String>,
}

impl Cp {
    pub fn new(targets: Vec<String>) -> Self {
        Self { targets }
    }

    pub fn execute(&self) -> Result<(), ShellError> {
        if self.targets.len() < 2 {
            return Err(ShellError::Other("cp: missing file operand".into()));
        }

        let dest = &self.targets[self.targets.len() - 1];
            let mut seen = HashSet::new();
            let mut duplicates = String::new();
        if self.targets.len() > 2 {
            if !Path::new(dest).is_dir() {
                return Err(ShellError::Other(format!(
                    "cp: target '{}' is not a directory",
                    dest
                ).into()));
            }

            for file in &self.targets[..self.targets.len() - 1] {
                if Path::new(file).is_file() {
                    if !seen.insert(file) {
                        duplicates.push_str(file)
                    }
                    let target_path = Path::new(dest).join(Path::new(file).file_name().unwrap());
                    fs::copy(file, target_path)?;
                } else {
                    println!("cp: '{}' is not a file", file);
                }
            }

            if !duplicates.is_empty() {
                 return Err(ShellError::Other(format!("cp: warning: source file '{}' specified more than once", duplicates).into()));
            }
            
        } else {
            let src = &self.targets[0];
            if src == dest {  
                 return Err(ShellError::Other(format!("cp: '{}' and '{}' are the same file", src, dest).into()));
            }
            if !Path::new(src).is_file() {
                return Err(ShellError::Other(format!("cp: '{}' is not a file", src).into()));
            }
            fs::copy(src, dest)?;
        }

        Ok(())
    }
}