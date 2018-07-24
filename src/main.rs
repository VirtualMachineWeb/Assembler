use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;


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

/*
    labels: HashMap<String, usize>, // label name to operation index
    operations: Vec<Operation>*/

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

/*
    let mut buffer = File::create(&args[2]).expect("Could not open output file");
    buffer.write(&bin[0..]).expect("Could not write to output file");*/
}