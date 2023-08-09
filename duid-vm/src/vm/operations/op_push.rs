use crate::{
    utils::{get_address},
    vm::{
        data::*,
        vm::DuidVm
    }
};


impl<const N: usize> DuidVm<N> {
    pub fn op_push(&mut self) {
        match self.pop_instructions(3) {
            Some(data) => {
                let address = get_address(data[0] as u16, data[1] as u16);
                let size = DataType::from([data[2]].as_slice()).get_size();
                match self.pop_memory(address, size) {
                    Some(m_data) => {
                        let mut value: Vec<u8> = Vec::with_capacity(size);
                        value.extend_from_slice(m_data);
                        self.push(&value);
                    },
                    None => {}
                }
            },
            None => {}
        }
    }
}
