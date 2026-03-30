mod cmd;
mod error;

use cmd::rm::Rm;
use cmd::mkdir::Mkdir;
use cmd::mv::Mv;
use cmd::cp::Cp;

use std::env::*;
use std::io::{self};
use rustyline::DefaultEditor;
use std::fs;

fn main() {
    // Initialize rustyline editor for input and history
    let mut rl = DefaultEditor::new().unwrap();
    let mut last_dir = current_dir().unwrap_or_else(|_| dirs::home_dir().unwrap());

    loop {
        // Safe current directory for prompt
        let cwd = match current_dir() {
            Ok(dir) => {
                last_dir = dir.clone();
                dir
            },
            Err(_) => last_dir.clone(),
        };
        let display_dir = cwd.to_string_lossy().replace(&var("HOME").unwrap_or_default(), "~");

        // Read input from user using rustyline
        let input: String = match rl.readline(&format!("\x1b[32m127.0.0.1@z01:\x1b[0m{}$ ", display_dir)) {
            Ok(line) => line,
            Err(_) => break, // Ctrl+D exits shell
        };

        if input.trim().is_empty() { 
            continue; 
        }

        rl.add_history_entry(input.trim()).unwrap();
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        let cmd = parts[0];
        let args = &parts[1..];

        match cmd {
            "exit" => break,
            "echo" => println!("{}", args.join(" ")),
            "pwd" => println!("{}", current_dir().unwrap().display()),
            "cat" => {
                if args.is_empty() {
                    let mut line = String::new();
                    while io::stdin().read_line(&mut line).unwrap() > 0 {
                        print!("{}", line);
                        line.clear();
                    }
                } else {
                    for file in args {
                        match fs::read_to_string(file) {
                            Ok(content) => print!("{}", content),
                            Err(_) => println!("cat: {}: No such file or directory", file),
                        }
                    }
                }
            },
            "mv" => {
                let mv_args: Vec<String> = args.iter().map(|s| s.to_string()).collect();
                let mv_cmd = Mv::new(mv_args);
                if let Err(e) = mv_cmd.execute() {
                    eprintln!("{}", e);
                }
            },
            "cp" => {
                let cp_args: Vec<String> = args.iter().map(|s| s.to_string()).collect();
                let cp_cmd = Cp::new(cp_args);
                if let Err(e) = cp_cmd.execute() {
                    eprintln!("{}", e);
                }
            },
            "rm" => {
                let rm_args: Vec<String> = args.iter().map(|s| s.to_string()).collect();
                let rm_cmd = Rm::new(rm_args);
                if let Err(e) = rm_cmd.execute() {
                    eprintln!("{}", e);
                }
            },
            "mkdir" => {
                let mkdir_args: Vec<String> = args.iter().map(|s| s.to_string()).collect();
                let mkdir_cmd = Mkdir::new(mkdir_args, cwd);
                if let Err(e) = mkdir_cmd.execute() {
                    eprintln!("{}", e);
                }
            },
            "cd" => {
                let new_dir = if args.is_empty() {
                    dirs::home_dir().unwrap_or_else(|| std::path::PathBuf::from("/"))
                } else {
                    if args.len() != 1 {
                        println!("cd: too many arguments");
                        continue;
                    }
                    std::path::PathBuf::from(args[0])
                };

                if let Err(e) = set_current_dir(&new_dir) {
                    println!("cd: {}: {}", new_dir.display(), e);
                }
            },
            _ => println!("Command '{}' not found", cmd),
        }
    }
}