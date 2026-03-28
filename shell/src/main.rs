//use std::env::*;
use std::io::{ Write};
use std::io::*;
//use shell::*;
 
fn main()  {
    println!(r#"███████╗██╗  ██╗███████╗██╗     ██╗     
██╔════╝██║  ██║██╔════╝██║     ██║     
███████╗███████║█████╗  ██║     ██║     
╚════██║██╔══██║██╔══╝  ██║     ██║     
███████║██║  ██║███████╗███████╗███████╗
╚══════╝╚═╝  ╚═╝╚══════╝╚══════╝╚══════╝"#);

let mut input = String::new();
loop {
    print!("@Trivia_Shell:$ ");
    stdout().flush().unwrap(); 
    
    input.clear();
    stdin().read_line(&mut input).unwrap();
    
    let exec = input.trim().replace('\x1b', "").split_whitespace().collect::<Vec<_>>();
    let mut cmd = exec[0];
    let args = &exec[1..];
    println!("{} -- {:?}", cmd, args);
        // match input.as_str() {
        //     "exit" => break ,
        //     "echo" => println!(""),  
        //     _ if input.starts_with("rm") => {
        //         let mut rm = input.split_whitespace();
        //         rm.next();
        //         let args: Vec<String> = rm.map(|ch| ch.to_string()).collect();
        //         println!("{:?}", Rm::new(args));
        //     }
        //     _ if input.starts_with("cat") => {
        //         let mut parts = input.split_whitespace();
        //         parts.next(); 
        //         let files: Vec<&str> = parts.collect();

        //         if files.is_empty() {
        //             let mut line = String::new();
        //             while stdin().read_line(&mut line).unwrap() > 0 {
        //                 print!("{}", line);
        //                 line.clear();
        //             }
        //         } else {
        //             for file in files {
        //                 match std::fs::read_to_string(file) {
        //                     Ok(content) => print!("{}", content),
        //                     Err(_) => println!("cat: {}: No such file or directory", file),
        //                 }
        //             }
        //         }
        //     }

        //     _ if input.starts_with("pwd") => {
        //         println!("{}", current_dir().unwrap().display());
        //     }

        //     _ if input.starts_with("echo ") => {
        //             println!("{}", input.strip_prefix("echo ").unwrap())
        //     }

        //     _ =>     println!("Command '{}' not found", input),
        
        // }   


    }
}
