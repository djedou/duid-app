/*
Instructions:
OpCode (u16) | Address (u32) | DataType (u16) | DataSize (u32)
    Or
OpCode | Address
    Or
OpCode | DataType
    Or
OpCode

    #############
OpCode = u16    // 0xXXXB => Opcode | Address | DataType | DataSize
                // 0xXXXC => Opcode | Address | DataSize
                // 0xXXXD => Opcode | DataType | DataSize
                // 0xXXXE => Opcode | DataSize
    ############
Address = u32 // Addres where the data is located in Memory
    ###########
DataType = u16  // DataType of the data in Memory for this specific Instruction;
DataSize = u32  // DataSize of the data in Memory for this specific Instruction;
*/


#[derive(Debug, Copy, Clone)]
// ANCHOR: vm_opcode
pub enum Flag {
    None, // 0x00
    SignOn, // 0x01
    SignOff, // 0x02
    ZeroOn, // 0x03
    ZeroOff, // 0x04
    CarryOn, // 0x05
    CarryOff, // 0x06
}

impl From<Flag> for u8 {
    fn from(value: Flag) -> u8 {
        match value {
            Flag::None => 0x00,
            Flag::SignOn => 0x01,
            Flag::SignOff => 0x02,
            Flag::ZeroOn => 0x03,
            Flag::ZeroOff => 0x04,
            Flag::CarryOn => 0x05,
            Flag::CarryOff => 0x06,
        }
    }
}

#[derive(Debug, Copy, Clone)]
// ANCHOR: vm_opcode
pub enum OpCode {
    //### Stack Operations ###
    PUSH, // 
    POP,
    UPDATE,
    //### Control Flow ###
    CMP, // Pop the top two locations, perform compare value, and set flag. (Sign , Zero)
    JMP, // Unconditionally jump to the instruction at address label
    JMR, // Jump relate
    JS, // Jump if sign flag is On
    JNS, // Jump if sign flag is Off
    JZ, // Jump if zero flag is On
    JNZ, // Jump if zero flag is Off
    JC, // Jump if carry flag is On
    JNC, // Jump if carry flag is Off
    //### Arithmetic ###
    OpAdd,
    OpMinus,
    OpStar,
    OpSlash,
    OpPercent,
    OpBitAnd,
    OpBitOr,
    OpBitXor,
    OpShl,
    OpShr,
    OpUMinus,
    OpUNot,
    OpEqEq,
    OpNe,
    OpGt,
    OpLt,
    OpGe,
    OpLe,
    OpOrOr,
    OpAndAnd,
    OpUAnd,
    //### Others ###
    OpReturn,
    OpOutput
}
// ANCHOR_END: vm_opcode

pub fn make_op(op: OpCode) -> u16 {
    match op {
        // Stack Operations
        OpCode::PUSH => 0x001C, // C => Opcode | Address | DataSize
        OpCode::POP => 0x002E, // E => Opcode;DataSize
        OpCode::UPDATE => 0x003C, // C => Opcode | Address | DataSize
        // Control Flow
        OpCode::CMP => 0x004D, // D => Opcode;DataType;DataSize 
        OpCode::JMP => 0x005C, // C => Opcode | Address | DataSize
        OpCode::JMR => 0x006C, 
        OpCode::JS => 0x007C, 
        OpCode::JNS => 0x008C, 
        OpCode::JZ => 0x009C, 
        OpCode::JNZ => 0x00AC, 
        OpCode::JC => 0x00BC, 
        OpCode::JNC => 0x00CC, 
        // Arithmetic And Logic
        OpCode::OpAdd => 0x00DD,
        OpCode::OpMinus => 0x00ED,
        OpCode::OpStar => 0x00FD,
        OpCode::OpSlash => 0x010D,
        OpCode::OpPercent => 0x011D,
        OpCode::OpBitAnd => 0x012D,
        OpCode::OpBitOr => 0x013D,
        OpCode::OpBitXor => 0x014D,
        OpCode::OpShl => 0x015D,
        OpCode::OpShr => 0x016D,
        OpCode::OpUMinus => 0x017D,
        OpCode::OpUNot => 0x018D,
        OpCode::OpEqEq => 0x019D,
        OpCode::OpNe => 0x01AD,
        OpCode::OpGt => 0x01BD,
        OpCode::OpLt => 0x01CD,
        OpCode::OpGe  => 0x01DD,
        OpCode::OpLe => 0x01ED,
        OpCode::OpOrOr => 0x01FD,
        OpCode::OpAndAnd => 0x020D,
        OpCode::OpUAnd => 0x021D,
        // System
        OpCode::OpReturn => 0xFF0C,
        OpCode::OpOutput => 0xFF1B, //B => Opcode | Address | DataType | DataSize
    }
}