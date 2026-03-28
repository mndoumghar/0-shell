pub use std::fs;
pub use std::path::Path;


#[derive(Debug, Clone)]
pub struct Rm {
    pub args: Vec<String>,
    pub flag: bool,
    pub targets: Vec<String>,
}

impl Rm {

    pub fn new(args: Vec<String>) -> Self {
        let mut flag = false;
        let mut targets = Vec::new();
        for arg in args.iter() {
            if arg == "-r" || arg == "-R" {
                flag = true;
            } else if arg.starts_with('-') {
                for c in arg[1..].chars() {
                    if c != 'r' && c != 'R' {
                        println!("flag: invalid option -- '{}'", c);
                    }
                }
                flag = true;
            } else {
                targets.push(arg.clone());
            } 
        }

        Self {
            args,
            flag,
            targets,
        }

    }

    
}