#![allow(unused)]

mod error_reporter;
mod scanner;

use std::{
    fs,
    io::{self, BufRead, Write},
    path::Path,
    process,
};

use clap::Parser;
use color_eyre::{eyre::eyre, Result};

use scanner::Scanner;

const REPL_FILE_NAME: &str = "REPL";

#[derive(Parser)]
struct Args {
    #[arg(index = 1)]
    file: Option<String>,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let args = Args::parse();

    match args.file {
        Some(file) => {
            if let Err(_) = run_file(file) {
                process::exit(65);
            }
        }
        None => run_prompt()?,
    }

    Ok(())
}

fn run_file(file: String) -> Result<()> {
    let file_name = match Path::new(&file).file_name() {
        Some(name) => name.to_str().unwrap(),
        None => return Err(eyre!("Invalid path")),
    };
    let program = fs::read_to_string(&file)?;
    let result = run(program, file_name.to_string());

    if result.had_error {
        Err(eyre!("A fatal error occured while running the program"))
    } else {
        Ok(())
    }
}

fn run_prompt() -> Result<()> {
    let mut stdout = io::stdout();
    print!("> ");
    stdout.flush()?;

    for line in io::stdin().lock().lines() {
        run(line?, REPL_FILE_NAME.to_string());
        print!("> ");
        stdout.flush()?;
    }

    Ok(())
}

fn run(program: String, file_name: String) -> Scanner {
    println!("{}", program);
    let program_scanner = {
        let mut var: Scanner = Default::default();
        var.data = program;
        var.file_name = file_name;
        var
    };
    let tokens = program_scanner.scan_tokens();

    program_scanner
}
