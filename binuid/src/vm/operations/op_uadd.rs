use crate::vm::vm::DuidVm;


impl<const N: usize> DuidVm<N> {
    pub fn op_uadd(&mut self) {
        match self.pop_instructions(3) {
            Some(_) => {},
            None => {}
        }
    }
}
