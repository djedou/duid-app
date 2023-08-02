use crate::compiler::vm::opcode::*;
use crate::compiler::vm::Bytecode;
//use crate::Node;
use super::instruction::Instruction;

// ANCHOR: vm
const STACK_SIZE: usize = 512;

pub struct DuidVm {
    //bytecode: Bytecode,
    ip: Instruction,
    stack: [u8; STACK_SIZE],
    //stack_ptr: usize, // points to the next free space
}
// ANCHOR_END: DuidVm

impl DuidVm {
    pub fn new(bytecode: Bytecode) -> Self {
        let mut stack = [0u8; STACK_SIZE];
        let byte_size = bytecode.code.len();
        if STACK_SIZE >= byte_size {
            stack[0..byte_size].iter_mut().zip(bytecode.code.iter()).for_each(|(a, b)| {*a = *b;});
        };


        Self {
            ip: Instruction::new(),
            stack,
        }
    }
    // ANCHOR: vm_interpreter
    pub fn run(&mut self) {
        /*
        let mut ip = 0; // instruction pointer
        while ip < self.bytecode.instructions.len() {
            let inst_addr = ip;
            ip += 1;

            match self.bytecode.instructions[inst_addr] {
                0x01 => {
                    //OpConst
                    let const_idx = convert_two_u8s_to_usize(
                        self.bytecode.instructions[ip],
                        self.bytecode.instructions[ip + 1],
                    );
                    ip += 2;
                    self.push(self.bytecode.constants[const_idx].clone());
                }
                0x02 => {
                    //OpPop
                    self.pop();
                }
                0x03 => {
                    // OpAdd
                    match (self.pop(), self.pop()) {
                        (Node::Int(rhs), Node::Int(lhs)) => self.push(Node::Int(lhs + rhs)),
                        _ => panic!("Unknown types to OpAdd"),
                    }
                }
                0x04 => {
                    // OpSub
                    match (self.pop(), self.pop()) {
                        (Node::Int(rhs), Node::Int(lhs)) => self.push(Node::Int(lhs - rhs)),
                        _ => panic!("Unknown types to OpSub"),
                    }
                }
                0x0A => {
                    // OpPlus
                    match self.pop() {
                        Node::Int(num) => self.push(Node::Int(num)),
                        _ => panic!("Unknown arg type to OpPlus"),
                    }
                }
                0x0B => {
                    // OpMinus
                    match self.pop() {
                        Node::Int(num) => self.push(Node::Int(-num)),
                        _ => panic!("Unknown arg type to OpMinus"),
                    }
                }
                _ => panic!("Unknown instruction"),
            }
        }*/
    }

    /*pub fn push(&mut self, node: Node) {
        self.stack[self.stack_ptr] = node;
        self.stack_ptr += 1; // ignoring the potential stack overflow
    }

    pub fn pop(&mut self) -> Node {
        // ignoring the potential of stack underflow
        // cloning rather than mem::replace for easier testing
        let node = self.stack[self.stack_ptr - 1].clone();
        self.stack_ptr -= 1;
        node
    }
    // ANCHOR_END: vm_interpreter
    pub fn pop_last(&self) -> &Node {
        // the stack pointer points to the next "free" space,
        // which also holds the most recently popped element
        &self.stack[self.stack_ptr]
    }*/

    pub fn push(&mut self, instr: Instruction) {
        /*let end = self.ip.start + value.len();
        self.stack[self.ip.start..end].iter_mut().zip(instr.data.iter()).for_each(|(a, b)| {*a = *b;});
        self.ip.next = end;
        self.ip.end = end - 1;*/
    }

    /*pub fn pop(&mut self, size: usize) -> Option<&[u8]> {
        match self.stack_top >= size {
            true => {
                let start = self.stack_top - size;
                let end = self.stack_top - 1;
                let data = &self.stack[start..=end];
                self.stack_top = start;
                Some(data)
            },
            false => None
        }
    }*/
}
/*
#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiler::vm::bytecode::Interpreter;
    use crate::Compile;

    fn assert_pop_last(source: &str, node: Node) {
        let byte_code = Interpreter::from_source(source);
        println!("byte code: {:?}", byte_code);
        let mut vm = DuidVm::new(byte_code);
        vm.run();
        //assert_eq!(&node, vm.pop_last());
    }

    #[test]
    fn unary() {
        assert_pop_last("+1", Node::Int(1));
        assert_pop_last("-2", Node::Int(-2));
    }

    #[test]
    fn binary() {
        assert_pop_last("1 + 2;", Node::Int(3));
        assert_pop_last("1 - 2;", Node::Int(-1));
    }
}
*/