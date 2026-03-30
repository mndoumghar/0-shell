mod cmd;
mod error;

use cmd::cp::Cp;
use cmd::mkdir::Mkdir;
use cmd::mv::Mv;
use cmd::rm::Rm;

use rustyline::DefaultEditor;
use std::env::*;
use std::fs;
use std::io;

fn main() {
    let mut rl = DefaultEditor::new().unwrap();
    let mut last_dir = current_dir().unwrap_or_else(|_| dirs::home_dir().unwrap());

    loop {
        // safe current directory for prompt
        let cwd = match current_dir() {
            Ok(dir) => { last_dir = dir.clone(); dir}
            Err(_) => last_dir.clone(),
        };

        let display_dir = cwd.to_string_lossy().replace(&var("HOME").unwrap_or_default(), "~");

        // read command line
        let input = match rl.readline(&format!("\x1b[32m127.0.0.1@z01:\x1b[0m{}$ ", display_dir)) {
            Ok(line) => line,
            Err(_) => break,
        };

        let input = input.trim();
        if input.is_empty() { continue;}

        rl.add_history_entry(input).unwrap();

        let parts: Vec<&str> = input.split_whitespace().collect();
        let cmd = parts[0];
        let args: Vec<String> = parts[1..].iter().map(ToString::to_string).collect();

        match cmd {
            "exit" => break,

            "echo" => {
                println!("{}", args.join(" "));
            }

            "pwd" => match current_dir() {
                Ok(dir) => println!("{}", dir.display()),
                Err(_) => println!("pwd: current directory not found"),
            },

            "cat" => {
                if args.is_empty() {
                    let mut line = String::new();
                    while io::stdin().read_line(&mut line).unwrap() > 0 {
                        print!("{}", line); line.clear();
                    }
                } else {
                    for file in &args {
                        match fs::read_to_string(file) {
                            Ok(content) => print!("{}", content),
                            Err(_) => { println!("cat: {}: No such file or directory", file) }
                        }
                    }
                }
            }

            "mv" => {
                if let Err(e) = Mv::new(args.clone()).execute() {
                    eprintln!("{}", e);
                }
            }

            "cp" => {
                if let Err(e) = Cp::new(args.clone()).execute() {
                    eprintln!("{}", e);
                }
            }

            "rm" => {
                if let Err(e) = Rm::new(args.clone()).execute() {
                    eprintln!("{}", e);
                }
            }

            "mkdir" => {
                if let Err(e) = Mkdir::new(args.clone(), cwd.clone()).execute() {
                    eprintln!("{}", e);
                }
            }

            "cd" => {
                let new_dir = if args.is_empty() {
                    dirs::home_dir().unwrap_or_else(|| std::path::PathBuf::from("/"))
                } else {
                    if args.len() != 1 { println!("cd: too many arguments"); continue;}
                    std::path::PathBuf::from(&args[0])
                };
                
                if let Err(e) = set_current_dir(&new_dir) { println!("cd: {}: {}", new_dir.display(), e) }
            }

            _ => println!("Command '{}' not found", cmd),
        }
    }
}