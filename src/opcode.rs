pub static OP_EAX: reg = 0;
pub static OP_ECX: reg = 1;
pub static OP_EDX: reg = 2;
pub static OP_EBX: reg = 3;
pub static OP_ESP: reg = 4;
pub static OP_EBP: reg = 5;
pub static OP_ESI: reg = 6;
pub static OP_EDI: reg = 7;

type reg = u8;

pub fn mov_reg(reg1: reg, reg2: reg) -> [u8; 2] {
    [0x89, 0xc0 + (reg1 * 7) + reg2]
}

pub fn mov_value(reg1: reg, value: u32) -> [u8; 5] {
    let value_u8: [u8; 4] = value.to_le_bytes();
    [
        0xb8 + reg1,
        value_u8[0],
        value_u8[1],
        value_u8[2],
        value_u8[3],
    ]
}

pub fn int(value: u8) -> [u8; 2] {
    [0xcd, value]
}

pub fn push(reg: u8) -> u8 {
    0x50 + reg
}
