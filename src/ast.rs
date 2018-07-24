pub struct Procedure {
    pub labels: Vec<(String, usize)>, // label name to operation index
    pub operations: Vec<Operation>
}

pub struct Tree {
    pub procedures: Vec<(String, Procedure)>
}

pub enum Operation {
    Jmp(Jmp),
    Jmps,
    JmpTrue(JmpTrue),
    CmpU8,
    Spi(Spi),
    Spd(Spd),
    PushU8(PushU8),
    PushU64(PushU64),
    PopU8,
    SetU8(SetU8),
    CplU8(CplU8),
    CpgU8(CpgU8),
    Halt
}

pub struct ExtProcRef {
    pub module: String,
    pub procedure: String
}

pub struct CpgU8 {
    pub address: Address
}

pub struct CplU8 {
    pub address: Address
}

pub struct Jmp {
    pub address: Address
}

pub struct JmpTrue {
    pub address: Address
}

pub struct PushU64 {
    pub address: Address
}

pub struct PushU8 {
    pub value: u8
}

pub struct SetU8 {
    pub address: Address
}

pub struct Spd {
    pub value: u64
}

pub struct Spi {
    pub value: u64
}

pub enum Address {
    Label(String),
    IntLiteral(u64),
    ProcRef(String),
    ExtProcRef(ExtProcRef)
}