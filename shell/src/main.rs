use std::env::*;
use std::io::{ Write};
use std::io::*;
// use shell::*;
 
fn main()  {

let mut input = String::new();
loop {
    print!("@Trivia_Shell:$ ");
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
