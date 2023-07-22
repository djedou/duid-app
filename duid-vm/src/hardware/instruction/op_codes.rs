use crate::hardware::stack_vm::StackVm;




#[derive(Debug)]
pub enum OpCode {
    Unreachable,
    Nop,
    Block,
    Loop,
    If,
    Else,
    End,
    Br,
    BrIf,
    BrTable,
    Return,
    Call,
    CallIndirect,
    Drop,
    Select,
    EqzU32
}

pub fn get_op_code(instruction: &u8) -> Option<OpCode> {
    match instruction {
        0x00 => Some(OpCode::Unreachable),
        0x01 => Some(OpCode::Nop),
        0x02 => Some(OpCode::Block),
        0x03 => Some(OpCode::Loop),
        0x04 => Some(OpCode::If),
        0x05 => Some(OpCode::Else),
        0x0B => Some(OpCode::End),
        0x0C => Some(OpCode::Br),
        0x0D => Some(OpCode::BrIf),
        0x0E => Some(OpCode::BrTable),
        0x0F => Some(OpCode::Return),
        0x10 => Some(OpCode::Call),
        0x11 => Some(OpCode::CallIndirect),
        0x1A => Some(OpCode::Drop),
        0x1B => Some(OpCode::Select),
        0x45 => Some(OpCode::EqzU32),
        /*15 => Some(OpCode::TRAP),
        15 => Some(OpCode::TRAP),
        15 => Some(OpCode::TRAP),
        15 => Some(OpCode::TRAP),
        15 => Some(OpCode::TRAP),*/
        _ => None,
    }
}


pub fn eqz_u32(vm: &mut StackVm<1024>) {
    match vm.pop(4) {
        Some(d) => {
            let value = u32::from_be_bytes(d.try_into().unwrap());
            match value == 0u32 {
                true => {
                    vm.push(1u32.to_be_bytes().as_slice());
                    vm.push(&[0x0F]);
                },
                false => {
                    vm.push(0u32.to_be_bytes().as_slice());
                    vm.push(&[0x0F]);
                }
            }
        }, 
        None => {

        }
    }
}


#[cfg(test)]
mod test_opcodes {
    use super::*;
    use crate::hardware::stack_vm::StackVm;
    use crate::hardware::instruction::*;

    #[test]
    fn test_eqz_u32() {
        let mut stack_vm = StackVm::<1024>::new();
        
        stack_vm.push(5552u32.to_be_bytes().as_slice());
        stack_vm.push(&[0x45]);

        execute_instruction(&mut stack_vm);
        println!("vm from eqz u32: {:?}", stack_vm);
    }


}