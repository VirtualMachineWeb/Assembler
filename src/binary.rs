use byteorder::{WriteBytesExt, BigEndian};

pub fn overwrite_u64(mut at: &mut [u8], value: &u64) {
    at.write_u64::<BigEndian>(*value).expect("Failed to overwrite u64");
}

pub fn write_u8(to: &mut Vec<u8>, value: u8) {
    to.write_u8(value).expect("Failed to write u8");
}

pub fn write_u16(to: &mut Vec<u8>, value: u16) {
    to.write_u16::<BigEndian>(value).expect("Failed to write u16");
}

pub fn write_u64(to: &mut Vec<u8>, value: u64) {
    to.write_u64::<BigEndian>(value).expect("Failed to write u64");
}