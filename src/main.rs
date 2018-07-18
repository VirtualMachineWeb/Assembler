use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

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

struct ExtProcRef {
    module: String,
    procedure: String
}

enum Token {
    IntLiteral(u64),
    NewLine,
    Proc,
    Opcode(Opcode),
    Label(String),
    LabelRef(String),
    ExtProcRef(ExtProcRef),
    End,
    EOF
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

fn text_to_end(text: &str) -> Option<Token> {
    if text == "end" {
        return Some(Token::End);
    } else {
        return None;
    }
}

fn text_to_label(text: &str) -> Option<Token> {
    if text.len() > 1 && text.ends_with(":") && text[0..text.len()-2].chars().all(|c| c.is_numeric() || c.is_lowercase()) && text.chars().next().unwrap().is_lowercase() {
        return Some(Token::Label(text[0..text.len()-1].to_string()));
    }  else {
        return None;
    }
}

fn text_to_labelref(text: &str) -> Option<Token> {
    if text.len() > 1 && text.starts_with("&") && text[1..text.len()-1].chars().all(|c| c.is_numeric() || c.is_lowercase()) && text.chars().nth(1).unwrap().is_lowercase() {
        return Some(Token::LabelRef(text[1..text.len()-1].to_string()));
    }  else {
        return None;
    }
}

fn text_to_extprocref(text: &str) -> Option<Token> {
    if text.len() > 3 && text.starts_with("&") {
        match text.find('.') {
            Some(dot_pos) => {
                let module = &text[1..dot_pos];
                let procedure = &text[dot_pos+1..];
                if  !module.chars().all(|c| c.is_numeric() || c.is_lowercase()) ||
                    !procedure.chars().all(|c| c.is_numeric() || c.is_lowercase()) ||
                    !module.chars().next().unwrap().is_lowercase() ||
                    !procedure.chars().next().unwrap().is_lowercase() {
                    return None;
                }
                return Some(Token::ExtProcRef(ExtProcRef{
                    module: module.to_string(),
                    procedure: procedure.to_string()
                }));
            },
            None => {
                return None;
            }
        }
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
    let converters = [
        text_to_intliteraldec,
        text_to_intliteralhex,
        text_to_proc,
        text_to_opcode,
        text_to_newline,
        text_to_label,
        text_to_labelref,
        text_to_extprocref,
        text_to_end
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
    tokens.push(Token::EOF);

    return tokens;
}

enum Operation {

}

enum Rule {
    Operation(Operation),
    Label(String)
}

struct Procedure {
    labels: HashMap<String, usize>, // label name to operation index
    operations: Vec<Operation>
}

struct Program {
    procedures: HashMap<String, Procedure>
}

fn parse_rule(source: &[Token]) -> Option<(Rule, &[Token])> {
    let mut source_leftover = source;

    return Some((Rule::Label("testlabel".to_string()), &source_leftover[1..])); // TODO
}

fn parse_proc(source: &[Token]) -> Option<(String, Procedure, &[Token])> {
    let mut source_leftover = source;

    while std::mem::discriminant(&source_leftover[0]) == std::mem::discriminant(&Token::NewLine) {
        source_leftover = &source_leftover[1..];
    }

    // check & remove proc label, save name
    match &source_leftover[0] {
        Token::Proc => {},
        _ => return None
    }
    let name: String;
    match &source_leftover[1] {
        Token::Label(label) => {
            name = label.to_string();
        },
        _ => return None
    }
    source_leftover = &source_leftover[2..];

    // parse all rules
    let mut labels: HashMap<String, usize> = HashMap::new();
    let mut operations: Vec<Operation> = Vec::new();    
    while std::mem::discriminant(&source_leftover[0]) != std::mem::discriminant(&Token::End) {
        match parse_rule(source_leftover) {
            Some((rule, leftover)) => {
                match rule {
                    Rule::Operation(operation) => {
                        operations.push(operation);
                    }, 
                    Rule::Label(label) => {
                        labels.insert(label, operations.len());
                    }
                }
                source_leftover = leftover;
            }
            None => return None
        }
    }
    
    // check & remove end proc
    if  source_leftover.len() < 2 || // end, proc
            std::mem::discriminant(&source_leftover[0]) != std::mem::discriminant(&Token::End) ||
            std::mem::discriminant(&source_leftover[1]) != std::mem::discriminant(&Token::Proc) {
        return None;
    }
    source_leftover = &source_leftover[2..];

    return Some((name, Procedure{labels: labels, operations: operations}, source_leftover));
}

fn parser(source: &Vec<Token>) -> Program {
    let mut procedures: HashMap<String, Procedure> = HashMap::new();

   let mut source_leftover = source.as_slice();
    while std::mem::discriminant(&source_leftover[0]) != std::mem::discriminant(&Token::EOF) {
        match parse_proc(source_leftover) {
            Some((name, procedure, leftover)) => {
                println!("proc: {}", name);
                procedures.insert(name, procedure);
                source_leftover = leftover;
            },
            None => panic!("Invalid procedure")
        }
    }

    return Program{procedures: procedures};
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

    let tokens = lexer(&contents);
    let program = parser(&tokens);
}