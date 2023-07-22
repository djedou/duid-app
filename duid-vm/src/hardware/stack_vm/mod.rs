
#[derive(Debug)]
pub struct StackVm<const N: usize> {
    stack: [u8; N],
    stack_top: usize
}


impl<const N: usize> StackVm<N> {
    pub fn new() -> Self {
        StackVm {
            stack: [0u8; N],
            stack_top: 0
        }
    }

    pub fn size(&self) -> usize {
        self.stack.len()
    }

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
}

#[cfg(test)]
mod stack_vm_test {
    use super::*;
    #[test]
    fn test_vm_debug() {
        let stack_vm = StackVm::<16>::new();
        println!("size: {:#?}", stack_vm.size());
        println!("{:?}", stack_vm);
    }

    #[test]
    fn test_vm_pop() {
        let mut stack_vm = StackVm::<16>::new();
        stack_vm.push(&[0x10, 0x15, 0xff]);
        println!("stack_top: {:#?}", stack_vm.stack_top);

        let data = stack_vm.pop(1);

        println!("");
        println!("################################");
        println!("data: {:?}", data);
        println!("vm size: {:#?}", stack_vm.size());
        println!("stack_top: {:#?}", stack_vm.stack_top);
        println!("{:?}", stack_vm.stack);

        let data2 = stack_vm.pop(2);

        println!("");
        println!("################################");
        println!("data2: {:?}", data2);
        println!("vm size: {:#?}", stack_vm.size());
        println!("stack_top: {:#?}", stack_vm.stack_top);
        println!("{:?}", stack_vm.stack);

        let data2 = stack_vm.pop(2);

        println!("");
        println!("################################");
        println!("data2: {:?}", data2);
        println!("vm size: {:#?}", stack_vm.size());
        println!("stack_top: {:#?}", stack_vm.stack_top);
        println!("{:?}", stack_vm.stack);
    }

    #[test]
    fn test_vm_push() {
        let mut stack_vm = StackVm::<16>::new();
        stack_vm.push(&[0x10, 0x15, 0xff]);
        
        println!("vm size: {:#?}", stack_vm.size());
        println!("stack_top: {:#?}", stack_vm.stack_top);
        println!("{:?}", stack_vm.stack);

        stack_vm.push(&[0x10, 0x15, 0xff]);
        println!("vm size: {:#?}", stack_vm.size());
        println!("stack_top: {:#?}", stack_vm.stack_top);
        println!("{:?}", stack_vm.stack);
    }
}