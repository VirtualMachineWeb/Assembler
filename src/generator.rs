use format_vmw;
use ast::{Operation, Tree, Address};
use vm::OpcodeValues;
use std::collections::HashMap;
use binary::*;

// returns: binary, offsets of procedures, addresses that require program offset, placeholders for external procedure calls
pub fn generate(source: &Tree) -> format_vmw::VMW {
    let mut bin: Vec<u8> = Vec::new();
    let mut procedures: HashMap<String, u64> = HashMap::new();
    let mut local_addresses: Vec<u64> = Vec::new();
    let mut external_procedures: Vec<(format_vmw::ExternalProcedure, u64)> = Vec::new();

    let mut itern_proc_place: Vec<(String, u64)> = Vec::new();

    for (name, procedure) in &source.procedures {
        if procedures.contains_key(name) {
            panic!("proc label already used");
        }
        procedures.insert(name.to_string(), bin.len() as u64);

        let mut label_offsets: HashMap<String, u64> = HashMap::new();
        let mut call_placeholders: Vec<(String, u64)> = Vec::new();
        let mut next_label: usize = 0;
        for (op_index, operation) in procedure.operations.iter().enumerate() {
            if next_label < procedure.labels.len() && op_index == procedure.labels[next_label].1 {
                if procedures.contains_key(&procedure.labels[next_label].0) {
                    panic!("proc label already used");
                }
                if label_offsets.contains_key(&procedure.labels[next_label].0) {
                    panic!("proc label already used");
                }
                label_offsets.insert(procedure.labels[next_label].0.to_string(), bin.len() as u64);
                next_label += 1;
            }

            // generate operation
            let (ref mut add_bin, ref mut add_call_placeholders, ref mut add_proccall_placeholders, ref mut add_extcall_placeholders) = generate_operation(bin.len() as u64, &operation);
            call_placeholders.append(add_call_placeholders);
            itern_proc_place.append(add_proccall_placeholders);
            external_procedures.append(add_extcall_placeholders);
            bin.append(add_bin);
        }

        for call_placeholder in call_placeholders {
            let maybe_offset = label_offsets.get(&call_placeholder.0);
            if !maybe_offset.is_some() {
                println!("{}", &call_placeholder.0);
                panic!("Could not find label");
            }
            local_addresses.push(call_placeholder.1);
            overwrite_u64(&mut bin[(call_placeholder.1 as usize)..], maybe_offset.unwrap())
        }
    }

    for proccall_placeholder in itern_proc_place {
        let maybe_offset = procedures.get(&proccall_placeholder.0);
        if !maybe_offset.is_some() {
            println!("{}", &proccall_placeholder.0);
            panic!("Could not find proc");
        }
        local_addresses.push(proccall_placeholder.1);
        overwrite_u64(&mut bin[(proccall_placeholder.1 as usize)..], maybe_offset.unwrap());
    }

    let mut procedures_vec: Vec<(String, u64)> = Vec::new();
    for (key, offset) in procedures {
        procedures_vec.push((key, offset));
    }

    let vmw: format_vmw::VMW = format_vmw::VMW::new(bin, procedures_vec, local_addresses, external_procedures);
    return vmw;
}

// returns: binary, addresses that require placeholders for procedure calls, placeholders for internal procedure calls, placeholders for external procedure calls
fn generate_operation(bin_offset: u64, operation: &Operation) -> (Vec<u8>, Vec<(String, u64)>, Vec<(String, u64)>, Vec<(format_vmw::ExternalProcedure, u64)>) {
    let mut bin: Vec<u8> = Vec::new();
    let mut call_placeholders: Vec<(String, u64)> = Vec::new();
    let mut proccall_placeholders: Vec<(String, u64)> = Vec::new();
    let mut extcall_placeholders: Vec<(format_vmw::ExternalProcedure, u64)> = Vec::new();
    
    match operation {
        Operation::CmpU8 => {
            write_u16(&mut bin, OpcodeValues::CmpU8 as u16);
        },
        Operation::CpgU8(data) => {
            write_u16(&mut bin, OpcodeValues::CpgU8 as u16);
            match &data.address {
                Address::Label(addr) => {
                    call_placeholders.push((addr.to_string(), bin_offset+bin.len() as u64));
                    write_u64(&mut bin, 0);
                },
                Address::IntLiteral(addr) => {
                    write_u64(&mut bin, *addr as u64);
                },
                Address::ProcRef(addr) => {
                    proccall_placeholders.push((addr.to_string(), bin_offset+bin.len() as u64));
                    write_u64(&mut bin, 0);
                },
                Address::ExtProcRef(addr) => {
                    extcall_placeholders.push((format_vmw::ExternalProcedure{module: addr.module.to_string(), procedure: addr.procedure.to_string()}, bin_offset+bin.len() as u64));
                    write_u64(&mut bin, 0);
                }
            }
        },
        Operation::CplU8(data) => {
            write_u16(&mut bin, OpcodeValues::CplU8 as u16);
            match &data.address {
                Address::Label(_) => {
                    panic!("TODO this should not accept label");
                },
                Address::IntLiteral(addr) => {
                    write_u64(&mut bin, *addr);
                },
                Address::ProcRef(_) => {
                    panic!("TODO this should not accept extlabel");
                }                
                Address::ExtProcRef(_) => {
                    panic!("TODO this should not accept extlabel");
                }
            }
        },
        Operation::Halt => {
            write_u16(&mut bin, OpcodeValues::Halt as u16);
        },
        Operation::Jmp(data) => {
            write_u16(&mut bin, OpcodeValues::Jmp as u16);
            match &data.address {
                Address::Label(addr) => {
                    call_placeholders.push((addr.to_string(), bin_offset+bin.len() as u64));
                    write_u64(&mut bin, 0);
                },
                Address::IntLiteral(addr) => {
                    write_u64(&mut bin, *addr as u64);
                },
                Address::ProcRef(addr) => {
                    proccall_placeholders.push((addr.to_string(), bin_offset+bin.len() as u64));
                    write_u64(&mut bin, 0);
                },
                Address::ExtProcRef(addr) => {
                    extcall_placeholders.push((format_vmw::ExternalProcedure{module: addr.module.to_string(), procedure: addr.procedure.to_string()}, bin_offset+bin.len() as u64));
                    write_u64(&mut bin, 0);
                }
            }
        },
        Operation::Jmps => {
            write_u16(&mut bin, OpcodeValues::Jmps as u16);
        },
        Operation::JmpTrue(data) => {
            write_u16(&mut bin, OpcodeValues::JmpTrue as u16);
            match &data.address {
                Address::Label(addr) => {
                    call_placeholders.push((addr.to_string(), bin_offset+bin.len() as u64));
                    write_u64(&mut bin, 0);
                },
                Address::IntLiteral(addr) => {
                    write_u64(&mut bin, *addr as u64);
                },
                Address::ProcRef(addr) => {
                    proccall_placeholders.push((addr.to_string(), bin_offset+bin.len() as u64));
                    write_u64(&mut bin, 0);
                },
                Address::ExtProcRef(addr) => {
                    extcall_placeholders.push((format_vmw::ExternalProcedure{module: addr.module.to_string(), procedure: addr.procedure.to_string()}, bin_offset+bin.len() as u64));
                    write_u64(&mut bin, 0);
                }
            }            
        },
        Operation::PopU8 => {
            write_u16(&mut bin, OpcodeValues::PopU8 as u16);
        },
        Operation::PushU64(data) => {
            write_u16(&mut bin, OpcodeValues::PushU64 as u16);
            match &data.address {
                Address::Label(addr) => {
                    call_placeholders.push((addr.to_string(), bin_offset+bin.len() as u64));
                    write_u64(&mut bin, 0);
                },
                Address::IntLiteral(addr) => {
                    write_u64(&mut bin, *addr as u64);
                },
                Address::ProcRef(addr) => {
                    proccall_placeholders.push((addr.to_string(), bin_offset+bin.len() as u64));
                    write_u64(&mut bin, 0);
                },
                Address::ExtProcRef(addr) => {
                    extcall_placeholders.push((format_vmw::ExternalProcedure{module: addr.module.to_string(), procedure: addr.procedure.to_string()}, bin_offset+bin.len() as u64));
                    write_u64(&mut bin, 0);
                }
            }
        },
        Operation::PushU8(data) => {
            write_u16(&mut bin, OpcodeValues::PushU8 as u16);
            write_u8(&mut bin, data.value);
        },
        Operation::SetU8(data) => {
            write_u16(&mut bin, OpcodeValues::SetU8 as u16);
            match &data.address {
                Address::Label(addr) => {
                    call_placeholders.push((addr.to_string(), bin_offset+bin.len() as u64));
                    write_u64(&mut bin, 0);
                },
                Address::IntLiteral(addr) => {
                    write_u64(&mut bin, *addr as u64);
                },
                Address::ProcRef(addr) => {
                    proccall_placeholders.push((addr.to_string(), bin_offset+bin.len() as u64));
                    write_u64(&mut bin, 0);
                },
                Address::ExtProcRef(addr) => {
                    extcall_placeholders.push((format_vmw::ExternalProcedure{module: addr.module.to_string(), procedure: addr.procedure.to_string()}, bin_offset+bin.len() as u64));
                    write_u64(&mut bin, 0);
                }
            }
        },
        Operation::Spd(data) => {
            write_u16(&mut bin, OpcodeValues::Spd as u16);
            write_u64(&mut bin, data.value);
        },
        Operation::Spi(data) => {
            write_u16(&mut bin, OpcodeValues::Spi as u16);
            write_u64(&mut bin, data.value);
        }
    }

    return (bin, call_placeholders, proccall_placeholders, extcall_placeholders);
}