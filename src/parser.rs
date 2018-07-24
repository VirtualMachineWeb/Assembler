use ast::*;
use lexer::{Token, Opcode};
use std;

pub fn parse(source: &Vec<Token>) -> Tree {
    let mut procedures: Vec<(String, Procedure)> = Vec::new();

   let mut source_leftover = source.as_slice();
    while std::mem::discriminant(&source_leftover[0]) != std::mem::discriminant(&Token::EOF) {
        match parse_proc(source_leftover) {
            Some((name, procedure, leftover)) => {
                procedures.push((name, procedure));
                source_leftover = leftover;
            },
            None => panic!("Invalid procedure")
        }
    }

    return Tree{procedures: procedures};
}

enum Rule {
    Operation(Operation),
    Label(String)
}

fn parse_cmp_u8(source: &[Token]) -> Option<(Rule, &[Token])> {
    return Some((Rule::Operation(Operation::CmpU8), source));
}

fn parse_cpg_u8(source: &[Token]) -> Option<(Rule, &[Token])> {
    match &source[0] {
        Token::IntLiteral(value) => {
            let op = Operation::CpgU8(CpgU8{address: Address::IntLiteral(*value)});
            return Some((Rule::Operation(op), &source[1..]));
        },
        Token::LabelRef(value) => {
            let op = Operation::CpgU8(CpgU8{address: Address::Label(value.to_string())});
            return Some((Rule::Operation(op), &source[1..]));
        },
        Token::ProcRef(value) => {
            let op = Operation::CpgU8(CpgU8{address: Address::ProcRef(value.to_string())});
            return Some((Rule::Operation(op), &source[1..]));
        },
        Token::ExtProcRef(value) => {
            let op = Operation::CpgU8(CpgU8{address: Address::ExtProcRef(ExtProcRef{module: value.module.to_string(), procedure: value.procedure.to_string() })});
            return Some((Rule::Operation(op), &source[1..]));
        },
        _ => {
            println!("parse_cpg_u8");
            return None;
        }
    }
}

fn parse_cpl_u8(source: &[Token]) -> Option<(Rule, &[Token])> {
    match &source[0] {
        Token::IntLiteral(value) => {
            let op = Operation::CplU8(CplU8{address: Address::IntLiteral(*value)});
            return Some((Rule::Operation(op), &source[1..]));
        },
        Token::LabelRef(value) => {
            let op = Operation::CplU8(CplU8{address: Address::Label(value.to_string())});
            return Some((Rule::Operation(op), &source[1..]));
        },
        Token::ProcRef(value) => {
            let op = Operation::CplU8(CplU8{address: Address::ProcRef(value.to_string())});
            return Some((Rule::Operation(op), &source[1..]));
        },
        Token::ExtProcRef(value) => {
            let op = Operation::CplU8(CplU8{address: Address::ExtProcRef(ExtProcRef{module: value.module.to_string(), procedure: value.procedure.to_string() })});
            return Some((Rule::Operation(op), &source[1..]));
        },
        _ => {
            println!("parse_cpl_u8");
            return None;
        }
    }
}

fn parse_halt(source: &[Token]) -> Option<(Rule, &[Token])> {
    return Some((Rule::Operation(Operation::Halt), source));
}

fn parse_jmp(source: &[Token]) -> Option<(Rule, &[Token])> {
    match &source[0] {
        Token::IntLiteral(value) => {
            let op = Operation::Jmp(Jmp{address: Address::IntLiteral(*value)});
            return Some((Rule::Operation(op), &source[1..]));
        },
        Token::LabelRef(value) => {
            let op = Operation::Jmp(Jmp{address: Address::Label(value.to_string())});
            return Some((Rule::Operation(op), &source[1..]));
        },
        Token::ProcRef(value) => {
            let op = Operation::Jmp(Jmp{address: Address::ProcRef(value.to_string())});
            return Some((Rule::Operation(op), &source[1..]));
        },
        Token::ExtProcRef(value) => {
            let op = Operation::Jmp(Jmp{address: Address::ExtProcRef(ExtProcRef{module: value.module.to_string(), procedure: value.procedure.to_string() })});
            return Some((Rule::Operation(op), &source[1..]));
        },
        _ => {
            println!("parse_jmp");
            return None;
        }
    }
}

fn parse_jmps(source: &[Token]) -> Option<(Rule, &[Token])> {
    return Some((Rule::Operation(Operation::Jmps), source));
}

fn parse_jmp_true(source: &[Token]) -> Option<(Rule, &[Token])> {
    match &source[0] {
        Token::IntLiteral(value) => {
            let op = Operation::JmpTrue(JmpTrue{address: Address::IntLiteral(*value)});
            return Some((Rule::Operation(op), &source[1..]));
        },
        Token::LabelRef(value) => {
            let op = Operation::JmpTrue(JmpTrue{address: Address::Label(value.to_string())});
            return Some((Rule::Operation(op), &source[1..]));
        },
        Token::ProcRef(value) => {
            let op = Operation::JmpTrue(JmpTrue{address: Address::ProcRef(value.to_string())});
            return Some((Rule::Operation(op), &source[1..]));
        },
        Token::ExtProcRef(value) => {
            let op = Operation::JmpTrue(JmpTrue{address: Address::ExtProcRef(ExtProcRef{module: value.module.to_string(), procedure: value.procedure.to_string() })});
            return Some((Rule::Operation(op), &source[1..]));
        },
        _ => {
            println!("parse_jmp_true");
            return None;
        }
    }
}

fn parse_pop_u8(source: &[Token]) -> Option<(Rule, &[Token])> {
    return Some((Rule::Operation(Operation::PopU8), source));
}

fn parse_push_u64(source: &[Token]) -> Option<(Rule, &[Token])> {
    match &source[0] {
        Token::IntLiteral(value) => {
            let op = Operation::PushU64(PushU64{address: Address::IntLiteral(*value)});
            return Some((Rule::Operation(op), &source[1..]));
        },
        Token::LabelRef(value) => {
            let op = Operation::PushU64(PushU64{address: Address::Label(value.to_string())});
            return Some((Rule::Operation(op), &source[1..]));
        },
        Token::ProcRef(value) => {
            let op = Operation::PushU64(PushU64{address: Address::ProcRef(value.to_string())});
            return Some((Rule::Operation(op), &source[1..]));
        },
        Token::ExtProcRef(value) => {
            let op = Operation::PushU64(PushU64{address: Address::ExtProcRef(ExtProcRef{module: value.module.to_string(), procedure: value.procedure.to_string() })});
            return Some((Rule::Operation(op), &source[1..]));
        },
        _ => {
            println!("parse_push_u64");
            return None;
        }
    }
}

fn parse_push_u8(source: &[Token]) -> Option<(Rule, &[Token])> {
    match &source[0] {
        Token::IntLiteral(value) => {
            let op = Operation::PushU8(PushU8{value: *value as u8});
            return Some((Rule::Operation(op), &source[1..]));
        },
        _ => {
            println!("parse_push_u8");
            return None;
        }
    }
}

fn parse_set_u8(source: &[Token]) -> Option<(Rule, &[Token])> {
    match &source[0] {
        Token::IntLiteral(value) => {
            let op = Operation::SetU8(SetU8{address: Address::IntLiteral(*value)});
            return Some((Rule::Operation(op), &source[1..]));
        },
        Token::LabelRef(value) => {
            let op = Operation::SetU8(SetU8{address: Address::Label(value.to_string())});
            return Some((Rule::Operation(op), &source[1..]));
        },
        Token::ProcRef(value) => {
            let op = Operation::SetU8(SetU8{address: Address::ProcRef(value.to_string())});
            return Some((Rule::Operation(op), &source[1..]));
        },
        Token::ExtProcRef(value) => {
            let op = Operation::SetU8(SetU8{address: Address::ExtProcRef(ExtProcRef{module: value.module.to_string(), procedure: value.procedure.to_string() })});
            return Some((Rule::Operation(op), &source[1..]));
        },
        _ => {
            println!("parse_set_u8");
            return None;
        }
    }
}

fn parse_spd(source: &[Token]) -> Option<(Rule, &[Token])> {
    match &source[0] {
        Token::IntLiteral(value) => {
            let op = Operation::Spd(Spd{value: *value});
            return Some((Rule::Operation(op), &source[1..]));
        },
        _ => {
            println!("parse_spd");
            return None;
        }
    }
}

fn parse_spi(source: &[Token]) -> Option<(Rule, &[Token])> {
    match &source[0] {
        Token::IntLiteral(value) => {
            let op = Operation::Spi(Spi{value: *value});
            return Some((Rule::Operation(op), &source[1..]));
        },
        _ => {
            println!("parse_spi");
            return None;
        }
    }
}


fn parse_rule(source: &[Token]) -> Option<(Rule, &[Token])> {
    match &source[0] {
        Token::Opcode(opcode) => {
            match opcode {
                Opcode::CmpU8 => {
                    return parse_cmp_u8(&source[1..]);
                },
                Opcode::CpgU8 => {
                    return parse_cpg_u8(&source[1..]);
                },
                Opcode::CplU8 => {
                    return parse_cpl_u8(&source[1..]);
                },
                Opcode::Halt => {
                    return parse_halt(&source[1..]);
                },
                Opcode::Jmp => {
                    return parse_jmp(&source[1..]);
                },
                Opcode::Jmps => {
                    return parse_jmps(&source[1..]);
                },
                Opcode::JmpTrue => {
                    return parse_jmp_true(&source[1..]);
                },
                Opcode::PopU8 => {
                    return parse_pop_u8(&source[1..]);
                },
                Opcode::PushU64 => {
                    return parse_push_u64(&source[1..]);
                },
                Opcode::PushU8 => {
                    return parse_push_u8(&source[1..]);
                },
                Opcode::SetU8 => {
                    return parse_set_u8(&source[1..]);
                },
                Opcode::Spd => {
                    return parse_spd(&source[1..]);
                },
                Opcode::Spi => {
                    return parse_spi(&source[1..]);
                }
            }
        },
        Token::Label(label) => {
            return Some((Rule::Label(label.to_string()), &source[1..]));
        },
        _ => {
            println!("nothing");
            return None;
        }
    }
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
    let mut labels: Vec<(String, usize)> = Vec::new();
    let mut operations: Vec<Operation> = Vec::new();    
    while std::mem::discriminant(&source_leftover[0]) != std::mem::discriminant(&Token::End) {
        while std::mem::discriminant(&source_leftover[0]) == std::mem::discriminant(&Token::NewLine) {
            source_leftover = &source_leftover[1..];
        }
        match parse_rule(source_leftover) {
            Some((rule, leftover)) => {
                match rule {
                    Rule::Operation(operation) => {
                        operations.push(operation);
                    }, 
                    Rule::Label(label) => {
                        labels.push((label, operations.len()));
                    }
                }
                source_leftover = leftover;
            }
            None => return None
        }
        while std::mem::discriminant(&source_leftover[0]) == std::mem::discriminant(&Token::NewLine) {
            source_leftover = &source_leftover[1..];
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