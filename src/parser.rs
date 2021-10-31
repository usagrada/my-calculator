#[derive(Parser, std::clone::Clone, std::marker::Copy)]
#[grammar = "./calc.pest"]
pub struct CalcParser;