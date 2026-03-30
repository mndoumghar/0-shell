mod cmd;
mod error;
use cmd::rm::Rm;
use cmd::mkdir::Mkdir;
use rustyline::DefaultEditor;
use std::env::*;
use std::fs;

fn main() {
    // Initialize rustyline editor for input and history
    let mut rl = DefaultEditor::new().unwrap();
    let mut last_dir = current_dir().unwrap_orelse(|| dirs::home_dir().unwrap());

    loop {
        // Safe current directory for prompt 
        let cwd = match current_dir() {
            Ok(dir) => { lastdir = dir.clone(); dir },
            Err() => last_dir.clone(),
        };
        let display_dir = cwd.to_string_lossy().replace(&var("HOME").unwrap_or_default(), "~");

        // Read input from user using rustyline
        let input: String = match rl.readline(&format!("\x1b[32m127.0.0.1@z01:\x1b[0m{}$ ", displaydir)) {
            Ok(line) => line,
            Err() => break, // Ctrl+D exits shell
        };
        if input.trim().isempty() { continue; }

        let = rl.add_historyentry(input.trim());
        let parts: Vec<> = input.trim().split_whitespace().collect();
        let cmd = parts[0];
        let args = &parts[1..];
        
