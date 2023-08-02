

#[derive(Debug, Clone)]
pub struct Instruction {
    pub start: usize,
    pub end: usize,
    pub next: usize,
    pub data: Vec<u8>
}

impl Instruction {
    pub fn new() -> Self {
        Instruction {
            start: 0,
            end: 0,
            next: 0,
            data: Vec::with_capacity(0)
        }
    }
}
