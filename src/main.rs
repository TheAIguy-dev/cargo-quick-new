use std::{env, fs, process::Command};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage: cargo-quick-new quick-new <file>");
        return;
    }

    let name: &str = &args[2];

    Command::new("cargo")
        .arg("new")
        .args(&args[2..])
        .status()
        .expect("Failed to run cargo new");

    fs::write(
        format!("{name}/Cargo.toml"),
        format!(
            r#"[package]
name = "{name}"
version = "0.1.0"
authors = ["TheAIguy_"]
edition = "2021"
description = ""
repository = "https://github.com/TheAIguy-dev/{name}"
license = "MIT"

[dependencies]
"#
        ),
    )
    .expect("Failed to write Cargo.toml");

    fs::write(
        format!("{name}/LICENSE"),
        r#"MIT License

Copyright (c) 2024 TheAIguy_

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
"#,
    )
    .expect("Failed to write LICENSE");

    fs::write(
        format!("{name}/README.md"),
        format!(
            r#"# {name}
"#
        ),
    )
    .expect("Failed to write README.md");

    fs::write(
        format!("{name}/.gitignore"),
        r#"/target
Cargo.lock
"#,
    )
    .expect("Failed to write .gitignore");
}
