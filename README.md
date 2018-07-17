 - create new exe project, --bin makes exe instead of lib, this creates a new folder, Cargo.toml is like package.json
    cargo new rust_calculater --bin
 - Run the program
    cargo run
 - Install dependencies by appending to the Cargo.toml, in vscode I would see errors, restarting vscode fixed that
    [dependencies]
    rand = "0.3.0"