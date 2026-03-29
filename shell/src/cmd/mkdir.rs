use crate::error::ShellError;
pub use std::fs;
pub use std::path::Path;

#[derive(Debug, Clone)]
pub struct Mkdir {
    pub targets: Vec<String>,
}

impl Mkdir {
    pub fn new(args: Vec<String>) -> Self {
        let mut targets = Vec::new();
        for arg in args.iter() {
            targets.push(arg.to_string());
        }
        Self {
            targets
        }
    }

    pub fn execute(&self) -> Result<(), ShellError> {
        if self.targets.is_empty() {
            return Err(ShellError::Other("mkdir: missing operand".into()));
        }
        for target in &self.targets {
            self.create_target(target)?;
        }
      
          Ok(())
    }

 fn create_target(&self, target: &str) -> Result<(), ShellError> {
        let path = Path::new(target);
        if path.exists() {
            eprintln!("mkdir: cannot creat directory '{}': File exists", target);
            return Ok(());
        }
        fs::create_dir_all(path)
        .map_err(|e| ShellError::Other(format!("mkdir: cannot create '{}': {}", target, e)))?;
        Ok(())
    }
}