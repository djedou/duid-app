use crate::{
    utils::get_address,
    vm::{
        data::*,
        vm::DuidVm
    }
};


impl<const N: usize> DuidVm<N> {
    pub fn op_return(&mut self) {
        match self.pop_instructions(3) {
            Some(data) => {
                let address = get_address(data[0] as u16, data[1] as u16);
                let size = DataType::from([data[2]].as_slice()).get_size();
                
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
