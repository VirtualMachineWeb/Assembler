use std::env;
use std::fs::File;
use std::io::prelude::*;

fn print_usage() {
    let args: Vec<String> = env::args().collect();
    println!("Usage: {} file", args[0]);
}

enum Opcode {
    Jmp,
    Jmps,
    JmpTrue,
    CmpU8,
    Spi,
    Spd,
    PushU8,
    PushU64,
    PopU8,
    SetU8,
    CplU8,
    CpgU8,
    Halt
}

enum Token {
    IntLiteral(u64),
    NewLine,
    Proc,
    Opcode(Opcode),
    Label(String)
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
                        return Some((&source[next_non_whitespace..next_non_whitespace+token_end], &source[next_non_whitespace+token_end..]));
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

fn text_to_intliteraldec(text: &str) -> Option<Token> {
    match text.parse::<u64>() {
        Ok(value) => {
            return Some(Token::IntLiteral(value));
        },
        Err(_) => {
            return None;
        }
    }
}

fn text_to_intliteralhex(text: &str) -> Option<Token> {
    if !text.starts_with("0x") {
        return None
    }
    match u64::from_str_radix(&text[2..], 16) {
        Ok(value) => {
            return Some(Token::IntLiteral(value));
        },
        Err(_) => {
            return None;
        }
    }
}

fn text_to_proc(text: &str) -> Option<Token> {
    if text == "proc" {
        return Some(Token::Proc);
    } else {
        return None;
    }
}

fn text_to_label(text: &str) -> Option<Token> {
    if text.len() > 1 && text.ends_with(":") && text[0..text.len()-2].chars().all(|c| c.is_numeric() || c.is_lowercase()) && text.chars().next().unwrap().is_lowercase() {
        return Some(Token::Label(text[0..text.len()-2].to_string()));
    }  else {
        return None;
    }
}


fn text_to_opcode(text: &str) -> Option<Token> {
    match text {
        "jmp" => Some(Token::Opcode(Opcode::Jmp)),
        "jmps" => Some(Token::Opcode(Opcode::Jmps)),
        "jmp_true" => Some(Token::Opcode(Opcode::JmpTrue)),
        "cmp_u8" => Some(Token::Opcode(Opcode::CmpU8)),
        "spi" => Some(Token::Opcode(Opcode::Spi)),
        "spd" => Some(Token::Opcode(Opcode::Spd)),
        "push_u8" => Some(Token::Opcode(Opcode::PushU8)),
        "push_u64" => Some(Token::Opcode(Opcode::PushU64)),
        "pop_u8" => Some(Token::Opcode(Opcode::PopU8)),
        "set_u8" => Some(Token::Opcode(Opcode::SetU8)),
        "cpl_u8" => Some(Token::Opcode(Opcode::CplU8)),
        "cpg_u8" => Some(Token::Opcode(Opcode::CpgU8)),
        "halt" => Some(Token::Opcode(Opcode::Halt)),
        _ => None
    }
}

fn text_to_newline(text: &str) -> Option<Token> {
    if text == "\n" {
        Some(Token::NewLine)
    } else {
        None
    }
}

fn text_to_token(text: &str) -> Option<Token> {
    println!("{}", text);
    let converters = [
        text_to_intliteraldec,
        text_to_intliteralhex,
        text_to_proc,
        text_to_opcode,
        text_to_newline,
        text_to_label
    ];
    for converter in converters.iter() {
        let token = converter(text);
        if token.is_some() {
            return Some(token.unwrap());
        }
    }
    return None;
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
            Some((token_text, leftover)) => {
                match text_to_token(token_text) {
                    Some(token) => tokens.push(token),
                    None => panic!("Invalid token: {}", token_text)
                }
                source_leftover = leftover;
            },
            None => {
                tokens_available = false;
            }
        }
    }

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