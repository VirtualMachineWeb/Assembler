use std::env;
use std::fs::File;
use std::io::prelude::*;

extern crate byteorder;
mod format_vmw;
mod parser;
mod ast;
mod generator;
mod vm;
mod binary;
mod lexer;

fn print_usage() {
    let args: Vec<String> = env::args().collect();
    println!("Usage: {} infile outfile", args[0]);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        print_usage();
        return;
    }

    let file: &String = &args[1];
    let mut f = File::open(file).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let tokens = lexer::lex(&contents);
    let ast = parser::parse(&tokens);

    let vmw: format_vmw::VMW = generator::generate(&ast);
    vmw.to_file(&args[2]);
}