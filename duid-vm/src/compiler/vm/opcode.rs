#[derive(Debug, Copy, Clone)]
// ANCHOR: vm_opcode
pub enum OpCode {
    OpReturn,
    OpAdd,
    OpMinus,
    OpStar,
    OpSlash,
    OpPercent,
    OpBitAnd,
    OpBitOr,
    OpBitXor,
    OpShl
}
// ANCHOR_END: vm_opcode

fn convert_u16_to_two_u8s(integer: u16) -> [u8; 2] {
    [(integer >> 8) as u8, integer as u8]
}

pub fn convert_two_u8s_to_usize(int1: u8, int2: u8) -> usize {
    ((int1 as usize) << 8) | int2 as usize
}

fn make_three_byte_op(code: u8, data: u16) -> Vec<u8> {
    let mut output = vec![code];
    output.extend(&convert_u16_to_two_u8s(data));
    output
}

pub fn make_op(op: OpCode) -> u8 {
    match op {
        OpCode::OpReturn => 0x05,
        OpCode::OpAdd => 0x10,
        OpCode::OpMinus => 0x11,
        OpCode::OpStar => 0x12,
        OpCode::OpSlash => 0x13,
        OpCode::OpPercent => 0x14,
        OpCode::OpBitAnd => 0x15,
        OpCode::OpBitOr => 0x16,
        OpCode::OpBitXor => 0x17,
        OpCode::OpShl => 0x18,
    }
}
