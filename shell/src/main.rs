use std::env::*;
use std::io::{ Write};
use std::io::*;
// use dirs::*; 
 
fn main()  {

let mut input = String::new();
loop {
    let cwd = current_dir().unwrap();
    let display_dir = cwd.to_string_lossy().replace(&var("HOME").unwrap_or_default(), "~");

    print!("\x1b[32m127.0.0.1@z01:\x1b[0m{}$ ", display_dir);
    stdout().flush().unwrap(); 
    
    input.clear();
    stdin().read_line(&mut input).unwrap();
    
    let cleaned = input.trim().replace('\x1b', "");
    let parts = cleaned.split_whitespace().collect::<Vec<_>>();
    if parts.is_empty() { continue; }

    let cmd = parts[0];
    let args = &parts[1..];

        match cmd {
            "exit" => break ,
            "echo" => { println!("{}", args.join(" "))},
            "pwd" =>  println!("{}", current_dir().unwrap().display()) ,  
            "cat" => {
                if args.is_empty() {
                    let mut line = String::new();
                    while stdin().read_line(&mut line).unwrap() > 0 {
                        print!("{}", line);
                        line.clear();
                    }
                } else {
                    for file in args {
                        match std::fs::read_to_string(file) {
                            Ok(content) => print!("{}", content),
                            Err(_) => println!("cat: {}: No such file or directory", file),
                        }
                    }
                }
            }

            "cd" => {
                let new_dir = if args.is_empty() {
                    dirs::home_dir().unwrap_or_else(|| std::path::PathBuf::from("/"))
                } else {
                    if args.len() != 1 { println!("cd: too many arguments"); continue;}
                    std::path::PathBuf::from(args[0])
                };

                if let Err(e) = set_current_dir(&new_dir) {
                    println!("cd: {}: {}", new_dir.display(), e);
                }
            },

            // _ if input.starts_with("rm") => {
            //     let mut rm = input.split_whitespace();
            //     rm.next();
            //     let args: Vec<String> = rm.map(|ch| ch.to_string()).collect();
            //     println!("{:?}", Rm::new(args));
            // }




            _ =>     println!("Command '{}' not found", cleaned),
        
        }   


    }
}
