use crate::hardware::stack_vm::StackVm;
use crate::hardware::instruction::*;

pub fn execute_instruction(vm: &mut StackVm<1024>) {
    match vm.pop(1) {
        Some(instr) => {
            
            match get_op_code(&instr[0]) {
                Some(OpCode::EqzU32) => eqz_u32(vm),
                /*Some(OpCode::AND) => and(instr, vm),
                Some(OpCode::NOT) => not(instr, vm),
                Some(OpCode::BR) => br(instr, vm),
                Some(OpCode::JMP) => jmp(instr, vm),
                Some(OpCode::JSR) => jsr(instr, vm),
                Some(OpCode::LD) => ld(instr, vm),
                Some(OpCode::LDI) => ldi(instr, vm),
                Some(OpCode::LDR) => ldr(instr, vm),
                Some(OpCode::LEA) => lea(instr, vm),
                Some(OpCode::ST) => st(instr, vm),
                Some(OpCode::STI) => sti(instr, vm),
                Some(OpCode::STR) => str(instr, vm),
                Some(OpCode::TRAP) => trap(instr, vm),*/
                _ => {}
            }
        },
        None => {}
    }
}