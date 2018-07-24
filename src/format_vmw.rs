pub struct VMW {
    bin: Vec<u8>,
    procedures: Vec<(String, u64)>,
    local_addresses: Vec<u64>,
    external_procedures: Vec<(ExternalProcedure, u64)>
}

pub struct ExternalProcedure {
    pub module: String,
    pub procedure: String
}

impl VMW {
    pub fn new(
        bin: Vec<u8>,
        procedures: Vec<(String, u64)>,
        local_addresses: Vec<u64>,
        external_procedures: Vec<(ExternalProcedure, u64)>)
        -> VMW {
        return VMW {
            bin: bin,
            procedures: procedures,
            local_addresses: local_addresses,
            external_procedures: external_procedures
        }
    }

    pub fn to_file(file: &str) {

    }

    /* TODO
    pub fn from_file(file: &str) {

    }*/
}