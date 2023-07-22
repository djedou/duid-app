#[derive(Debug)]
pub enum DataTypeOp {
    Num,
    Vec,
    Ref,
    Val,
    Res,
    Opt,
    ErH,
    Fun,
    Lim,
    Mem,
    Tab,
    Mut,
    Glo,
    Ext,
    Str
}

pub fn get_data_type_op_code(code: &u8) -> Option<DataTypeOp> {
    match code {
        0x00 => Some(DataTypeOp::Num),
        0x01 => Some(DataTypeOp::Vec),
        0x02 => Some(DataTypeOp::Ref),
        0x03 => Some(DataTypeOp::Val),
        0x04 => Some(DataTypeOp::Res),
        0x05 => Some(DataTypeOp::Opt),
        0x06 => Some(DataTypeOp::ErH),
        0x07 => Some(DataTypeOp::Fun),
        0x08 => Some(DataTypeOp::Lim),
        0x09 => Some(DataTypeOp::Mem),
        0x0A => Some(DataTypeOp::Tab),
        0x0B => Some(DataTypeOp::Mut),
        0x0C => Some(DataTypeOp::Glo),
        0x0D => Some(DataTypeOp::Ext),
        0x0E => Some(DataTypeOp::Str),
        _ => None
    }
}

pub fn get_data_type_byte(op: &DataTypeOp) -> u8 {
    match op {
        DataTypeOp::Num => 0x00,
        DataTypeOp::Vec => 0x01,
        DataTypeOp::Ref => 0x02,
        DataTypeOp::Val => 0x03,
        DataTypeOp::Res => 0x04,
        DataTypeOp::Opt => 0x05,
        DataTypeOp::ErH => 0x06,
        DataTypeOp::Fun => 0x07,
        DataTypeOp::Lim => 0x08,
        DataTypeOp::Mem => 0x09,
        DataTypeOp::Tab => 0x0A,
        DataTypeOp::Mut => 0x0B,
        DataTypeOp::Glo => 0x0C,
        DataTypeOp::Ext => 0x0D,
        DataTypeOp::Str => 0x0E
    }
}



#[cfg(test)]
mod data_type_op_test {
    use crate::hardware::stack_vm::StackVm;
    use super::*;

    #[test]
    fn test_data_type_op_pop() {
        let mut stack_vm = StackVm::<16>::new();
        stack_vm.push(&[
                get_data_type_byte(&DataTypeOp::Str), 
                get_data_type_byte(&DataTypeOp::Val), 
                get_data_type_byte(&DataTypeOp::Num)
            ]);
        
        let data1 = stack_vm.pop(1);
        let res = [0x00 as u8].as_slice();
        assert_eq!(data1, Some(res));
        
        let data2 = stack_vm.pop(1);
        let res = [0x03 as u8].as_slice();
        assert_eq!(data2, Some(res));

        let data3 = stack_vm.pop(1);
        let res = [0x0E as u8].as_slice();
        assert_eq!(data3, Some(res));
    }
}