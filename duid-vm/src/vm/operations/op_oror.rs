use crate::vm::{
    data::*,
    vm::DuidVm
};


impl<const N: usize> DuidVm<N> {
    pub fn op_oror(&mut self) {
        match self.pop_instructions(1) {
            Some(data) => {
                let data_type = DataType::from([data[0]].as_slice());
                let size = data_type.get_size();

                match data_type {
                    DataType::Bool => {
                        crate::OpLazyBoolBinary!(self, u8, size, ||);
                    },
                    _ => {}
                }
            },
            None => {}
        }
    }
}
