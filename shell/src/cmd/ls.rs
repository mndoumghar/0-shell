use crate::error::ShellError;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct Ls {
    pub targets: Vec<String>,
    pub token: bool,
    pub flag: Option<String>,
}

#[derive(Debug, Clone)]
pub enum Flag {
    F(),
    L(),
    A(),
}

impl Ls {
    pub fn new(args: Vec<String>) -> Self {
        let mut token = false;
        let mut targets = Vec::new();
        let mut flag: Option<String> = None;
        for arg in args.iter() {
            if arg == "-a"  || arg == "-F" || arg == "-l" {
                flag = Some(arg.clone());
                token = true;
            } else if arg.starts_with('-') {
                println!("ls: invalid option -- '{}'", arg);
            } else {
                targets.push(arg.clone());
            }
        }

        Self {
            token,
            targets,
            flag,
        }
    }

    pub fn execute(&self) -> Result<(), ShellError> {
        let paths = if self.targets.is_empty() {
            vec![".".to_string()]
        } else {
            self.targets.clone()
        };

        for target in paths {
            let path = Path::new(&target);
            if !path.exists() {
                println!("ls: cannot access '{}': No such file or directory", target);
                continue;
            }

            let entries = fs::read_dir(path)
                .map_err(|e| ShellError::from(e.to_string()))?;

            for entry in entries {
                let entry = entry.map_err(|e| ShellError::from(e.to_string()))?;
                let file_name = entry.file_name();
                print!("{}  ", file_name.to_string_lossy());
            }
            println!();
        }

        Ok(())
    }
}