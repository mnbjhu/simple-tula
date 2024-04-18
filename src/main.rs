use std::fs;

use clap::Parser;

use crate::cli::{lex::lex, parse::parse, run::run, Command};

pub mod cli;
pub mod lexer;
pub mod parser;
pub mod runtime;

fn main() {
    match Command::parse() {
        Command::Lex { file } => {
            let text = fs::read_to_string(&file).unwrap();
            let filename = file.to_str().unwrap();
            let _ = lex(filename, &text);
        }
        Command::Parse { file } => {
            let text = fs::read_to_string(&file).unwrap();
            let filename = file.to_str().unwrap();
            let _ = parse(filename, &text);
        }
        Command::Run { src, tape } => run(&src, &tape),
    }
}
