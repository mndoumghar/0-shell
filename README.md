0-shell

A modular, Unix-like shell written in Rust, designed as a systems programming project focused on implementing core shell behavior and file utilities without external command dependencies.

Project Overview

0-shell is a lightweight shell environment that reimplements common Unix commands internally using Rust’s standard library and low-level system abstractions.

The project emphasizes:

Clean modular architecture
Separation of command logic
Extensible command system
Unix-like behavior consistency
Safe systems programming in Rust

Unlike traditional shells, 0-shell does not rely on external binaries or subprocess execution for core commands. Every supported command is implemented natively.

Directory Structure

The project is organized into a command-centric architecture:

shell/
├── src/
│   ├── main.rs              # Shell entry point and REPL loop
│   ├── error.rs             # Unified error handling system
│   ├── cmd/                 # Built-in command implementations
│   │   ├── cat.rs
│   │   ├── cp.rs
│   │   ├── mkdir.rs
│   │   ├── mv.rs
│   │   ├── rm.rs
│   │   ├── help.rs
│   │   ├── mod.rs           # Command dispatcher / registry
│   │   └── ls/              # Modular ls implementation
│   │       ├── collector.rs # Directory traversal logic
│   │       ├── formatter.rs # Output formatting layer
│   │       ├── metadata.rs  # File metadata extraction
│   │       ├── parser.rs    # Flag parsing (-l, -a, -F)
│   │       ├── output.rs    # Display rendering
│   │       ├── types.rs     # Internal data structures
│   │       └── mod.rs       # ls orchestrator
│
├── Cargo.toml
├── Cargo.lock
├── Makefile
└── README.md
Architecture Design
1. Shell Core (main.rs)
Implements REPL loop
Reads user input
Delegates execution to command dispatcher
Handles exit conditions (exit, EOF)
2. Command System (cmd/)

All commands are implemented as independent modules.

Each command follows a consistent interface pattern:

Input parsing
Execution logic
Error propagation via error.rs

The system is designed for extensibility:
new commands can be added without modifying the core shell loop.

3. Command Dispatcher (cmd/mod.rs)

Acts as the central routing layer:

Maps input strings to command implementations
Handles unknown command resolution
Provides unified execution entry point

Example behavior:

Command "xyz" → Not found → Error handler
4. Error Handling (error.rs)

Centralized error system for:

File system errors
Invalid arguments
Command execution failures

Ensures consistent error output across all commands.

5. Advanced ls Implementation

The ls command is implemented as a mini subsystem, not a single function.

It is split into:

parser.rs → handles flags (-l, -a, -F)
collector.rs → reads directory contents
metadata.rs → extracts file metadata (permissions, size, etc.)
formatter.rs → transforms data into human-readable structure
output.rs → handles final rendering
types.rs → shared internal models

This design mirrors real-world implementations like GNU coreutils.

Supported Commands
File System Commands
cat – read file contents
cp – copy files
mv – move or rename files
rm – remove files (including recursive mode)
mkdir – create directories
Directory Navigation
cd – change directory (implemented in shell core)
pwd – print working directory
ls – directory listing with flags:
-l long format
-a include hidden files
-F classify file types
Utility
echo – print text to stdout
help – display built-in command reference
exit – terminate shell session
Execution Model

0-shell follows a strict execution pipeline:

Input → Parse → Dispatch → Execute → Return

Key properties:

Synchronous execution (no background jobs)
No external process spawning for built-ins
Immediate return to prompt after completion
Build & Run
Build
cargo build --release
Run
cargo run
Example Session
$ pwd
/home/user/0-shell

$ ls -l
total 12
drwxr-xr-x  src
-rw-r--r--  Cargo.toml

$ cat README.md
...

$ mkdir test
$ cd test
$ pwd
/home/user/0-shell/test

$ unknown
Command 'unknown' not found

$ exit
Design Philosophy

This project follows a systems-first design approach:

Modular over monolithic design
Explicit control flow over abstraction hiding
Unix behavioral consistency over convenience
Rust safety guarantees over manual memory control

The architecture intentionally mirrors real Unix utilities while remaining fully self-contained.

Constraints
No external binary execution (no Command::new for system tools)
No piping, redirection, or globbing in base version
Only built-in command implementations allowed
Must handle invalid input safely without crashing
Future Improvements
Command history and navigation
Signal handling (Ctrl+C)
Pipes and redirection system
Environment variable expansion
Plugin-based command architecture
Tab completion system
Colored terminal output
Author Intent

0-shell is designed as a bridge between:

High-level Rust programming
Low-level Unix system behavior
Real-world shell architecture understanding