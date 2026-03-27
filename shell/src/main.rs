use std::io::{ Write};
use std::io::*;

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
    
    
    if input.trim_end() == "exit" {
        break;
    }  
    println!("--------> {}", input.trim_end());
       

}
}
