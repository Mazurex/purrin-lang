// Copyright 2026 Maz
// Licensed under the Apache License, Version 2.0

pub mod error;
pub mod lexer;
pub mod span;
pub mod cursor;
pub mod file;

use crate::lexer::lexer::Lexer;
use std::{env, fs, process};
use crate::file::SourceFile;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Invalid usage! Correct usage is: ./purrin myfile.prn");
        process::exit(1);
    }

    let filepath = &args[1];

    let src = fs::read_to_string(filepath).expect("File doesn't exist lmao");

    let source_file = SourceFile::new(filepath.to_string(), src);

    let mut lexer = Lexer::new(source_file.clone());
    let tokens = match lexer.tokenize() {
        Ok(tokens) => tokens,
        Err(e) => {
            println!("{}", e.format());
            process::exit(1);
        }
    };

    for token in &tokens {
        println!("{}", token.as_str());
    }
}
