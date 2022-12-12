use std::collections::VecDeque;

#[derive(Debug, Copy, Clone)]
pub enum Operand {
    Value(u64),
    Old,
}

#[derive(Debug, Copy, Clone)]
pub enum Operation {
    Mul(Operand, Operand),
    Add(Operand, Operand),
}

#[derive(Debug)]
pub struct Monkey {
    pub id: u64,
    pub items: VecDeque<u64>,
    pub operation: Operation,
    pub test: u64,
    pub throw_true: u64,
    pub throw_false: u64,
    pub inspect_count: u64,
}
