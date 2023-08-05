
#[macro_export]
macro_rules! OpArithOrLog {
    ($stack:expr, $data_type:ty, $data:expr, $size:literal, $value:expr, $op:tt ) => {
        let rhs = <$data_type>::from_be_bytes($data[..$size].try_into().unwrap());
        let lhs = <$data_type>::from_be_bytes($data[$size..].try_into().unwrap());
        $stack.push(&(lhs $op rhs).to_be_bytes());
        $stack.push(&[u8::from(&$value), make_op($crate::compiler::vm::opcode::OpCode::OpReturn)]);
    };
}

#[macro_export]
macro_rules! OpArithOrLogFloat {
    ($stack:expr, $data_type:ty, $data_type_int:ty, $data:expr, $size:literal, $value:expr, $op:tt ) => {
        let rhs = <$data_type>::from_bits(<$data_type_int>::from_be_bytes($data[..$size].try_into().unwrap()));
        let lhs = <$data_type>::from_bits(<$data_type_int>::from_be_bytes($data[$size..].try_into().unwrap()));
        $stack.push(&(lhs $op rhs).to_bits().to_be_bytes());
        $stack.push(&[u8::from(&$value), make_op($crate::compiler::vm::opcode::OpCode::OpReturn)]);
    };
}

#[macro_export]
macro_rules! ByteCodeFromDataValue {
    ($stack:expr, $rhs:expr, $lhs:expr, $data_type:expr, $op:expr ) => {
        $stack.bytecode.code.extend_from_slice(&$rhs);
        $stack.bytecode.code.extend_from_slice(&$lhs);
        $stack.bytecode.code.extend_from_slice(&[u8::from(&$data_type), make_op($op)]);
    }
}