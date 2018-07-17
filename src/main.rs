use std::env;
use std::fs::File;
use std::io::prelude::*;

fn print_usage() {
    let args: Vec<String> = env::args().collect();
    println!("Usage: {} file", args[0]);
}

struct IntLiteral {
    value: u32
}

enum Token {
    IntLiteral(IntLiteral),
    NewLine,
    Proc
}

// returns: token, leftover src
fn next_token_text(source: &str) -> Option<(&str, &str)> {
    match source.find(|c: char| return c != ' ' && c != '\t') {
        Some(next_non_whitespace) => {
            if source[next_non_whitespace..].starts_with('\n') {
                return Some((&source[next_non_whitespace..(next_non_whitespace+1)], &source[next_non_whitespace+1..]));
            }
            else {
                match source[next_non_whitespace..].find(|c: char| return c == ' ' || c == '\t' || c == '\n') {
                    Some(token_end) => {
                        return Some((&source[next_non_whitespace..token_end], &source[token_end..]));
                    },
                    None => {
                        return Some((&source[next_non_whitespace..], ""));
                    }
                }
            }
        },
        None => {
            return None;
        }
    }
}

fn convert_newlines(source: &str) -> String {
    let mut result = source.replace("\r\n", "\n");
    result = result.replace("\n\r", "\n");
    result = result.replace("\r", "\n");
    // TODO replace multiple newlines
    return result;
}

fn lexer(source: &str) -> Vec<Token> {
    let mut tokens = Vec::new();

    let mut tokens_available = true;
    let mut source_leftover: &str = &convert_newlines(source);
    while tokens_available {
        let token_res = next_token_text(source_leftover);
        match token_res {
            Some((token, leftover)) => {
                println!("token: {}", token);
                source_leftover = leftover;
            },
            None => {
                tokens_available = false;
            }
        }
    }

/*
    let mut pos: usize = 0;
    let mut tokens_available = true;
    while tokens_available {
        match source.find(|c: char| return c != ' ' && c != '\t') {
            Some(next_non_whitespace) => {
                if source[next_non_whitespace..].starts_with("proc") && source[(next_non_whitespace+"proc".len())..] == ' ' {
                    tokens.push(Token::Proc);
                    pos += "proc".len();
                }
            },
            None => {
                tokens_available = false;
            }
        }
    }*/
    return tokens;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        print_usage();
        return;
    }

    let file: &String = &args[1];
    let mut f = File::open(file).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    lexer(&contents);
}