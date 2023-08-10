
#[derive(Debug)]
pub struct DuidVm<const N: usize> {
    memory: Vec<u8>,
    stack: [u8; N],
    stack_top: usize,
    instructions: Vec<u16>,
    inst_size: usize,
    ip: usize,
    flag: u8
}


impl<const N: usize> DuidVm<N> {
    pub fn new() -> Self {
        DuidVm {
            memory: Vec::with_capacity(0),
            stack: [0u8; N],
            stack_top: 0,
            instructions: Vec::with_capacity(0),
            inst_size: 0,
            ip: 0,
            flag: 0x00
        }
    }

    
    //############ Memory op start #####################
    pub fn load_memory(&mut self, value: &[u8]) {
        self.memory.extend_from_slice(value);
    }

    pub fn pop_memory(&mut self, address: usize, size: usize) -> Option<&[u8]> {
        if size == 1 {
            let end = address + size;
            let data = &self.memory[address..end];
            Some(data)
        }
        else if size > 1 {
            let end = address + size;
            let data = &self.memory[address..end];
            Some(data)
        }
        else {
            None
        }
    }

    pub fn update_memory(&mut self, address: usize, value: &[u8]) {
        let end = address + value.len();
        self.memory[address..end].iter_mut().zip(value.iter()).for_each(|(a, b)| {*a = *b;});
    }
    //############ Memory op end #####################

    //############ Instruction op start #####################
    pub fn load_instructions(&mut self, value: &[u16]) {
        self.instructions.extend_from_slice(value);
        self.inst_size = self.inst_size + value.len();
    }

    pub fn pop_instructions(&mut self, size: usize) -> Option<&[u16]> {
        let end = self.ip + size;
        match self.inst_size >= end {
            true => {
                let start = self.ip;
                let data = &self.instructions[start..end];
                self.ip = end;
                Some(data)
            },
            false => None
        }
    }
    //############ Instruction op end #####################


    //############ Stack op start #####################
    pub fn push(&mut self, value: &[u8]) {
        let end = self.stack_top + value.len();
        self.stack[self.stack_top..end].iter_mut().zip(value.iter()).for_each(|(a, b)| {*a = *b;});
        self.stack_top = end;
    }

    pub fn pop(&mut self, size: usize) -> Option<&[u8]> {
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
    }
    //############ Stack op end #####################

    pub fn run(&mut self) {
        // Get Instructions
        while let Some(instr) = self.pop_instructions(1) {
            match instr {
                [0x001B] => {
                    self.op_push();
                },
                [0x00DD] => {
                    self.op_add();
                },
                [0x00ED] => {
                    self.op_minus();
                },
                [0x00FD] => {
                    self.op_star();
                },
                [0x010D] => {
                    self.op_slash();
                },
                [0x011D] => {
                    self.op_percent();
                },
                [0x012D] => {
                    self.op_bit_and();
                },
                [0x013D] => {
                    self.op_bit_or();
                },
                [0x014D] => {
                    self.op_bit_xor();
                },
                [0x015D] => {
                    self.op_shl();
                },
                [0x016D] => {
                    self.op_shr();
                },
                [0x017D] => {
                    self.op_uminus();
                },
                [0x018D] => {
                    self.op_not();
                },
                [0x019D] => {
                    self.op_eqeq();
                },
                [0x01AD] => {
                    self.op_ne();
                },
                [0x01BD] => {
                    self.op_gt();
                },
                [0x01CD] => {
                    self.op_lt();
                },
                [0x01DD] => {
                    self.op_ge();
                },
                [0x01ED] => {
                    self.op_le();
                },
                [0xFF0B] => {
                    self.op_return();
                },
                [0xFF1B] => {
                    self.op_output();
                },
                _ => {}
            }
        }
    }
}