use crate::{
    utils::{get_usize_from_u16s},
    vm::vm::DuidVm
};


impl<const N: usize> DuidVm<N> {
    pub fn op_return(&mut self) {
        match self.pop_instructions(4) {
            Some(data) => {
                let address = get_usize_from_u16s(data[0] as u16, data[1] as u16);
                let size = get_usize_from_u16s(data[2] as u16, data[3] as u16);
                
                match &mut self.pop(size) {
                    Some(value) => {
                        let mut new_value: Vec<u8> = Vec::with_capacity(size);
                        new_value.extend_from_slice(value);
                        self.update_memory(address, &new_value);
                    },
                    _ => {}
                }
            },
            None => {}
        }
    }
}
