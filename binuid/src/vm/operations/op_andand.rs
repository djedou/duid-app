use crate::{
    utils::{get_usize_from_u16s},
    vm::{
        data::*,
        vm::DuidVm
    }
};


impl<const N: usize> DuidVm<N> {
    pub fn op_andand(&mut self) {
        match self.pop_instructions(3) {
            Some(data) => {
                let data_type = DataType::from([data[0]].as_slice());
                let size = get_usize_from_u16s(data[1] as u16, data[2] as u16); 

                match data_type {
                    DataType::BoolType => {crate::OpLazyBoolBinary!(self, u8, size, &&);},
                    _ => {},
                }
            },
            None => {}
        }
    }
}
